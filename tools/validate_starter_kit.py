#!/usr/bin/env python3
"""Validate the Topology starter kit's machine-readable contracts.

This checks syntax, work-item schema conformance, ID uniqueness, dependency
existence/acyclicity, evidence-directory naming, and synchronization between
work-items/index.yaml and the packet files.

Dependencies: PyYAML and jsonschema.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
import tomllib
from collections import Counter, defaultdict, deque
from pathlib import Path
from typing import Any

try:
    import jsonschema
    import yaml
except ImportError as exc:  # pragma: no cover - environment diagnostic
    print(
        "Missing validation dependency. Install PyYAML and jsonschema, then retry: "
        "python -m pip install PyYAML jsonschema",
        file=sys.stderr,
    )
    raise SystemExit(2) from exc


ALLOWED_REQUIREMENT_PREFIXES = {
    "DEV",
    "PRESET",
    "GRAPH",
    "BLOCK",
    "SCENE",
    "PERF",
    "UTILITY",
    "CAB",
    "FC",
    "LIB",
    "UNDO",
    "PACK",
    "TRANSPORT",
    "SIM",
    "CAPTURE",
    "AI",
    "A11Y",
    "UI",
    "PRIV",
    "SEC",
    "PLAT",
    "QA",
}


def load_yaml(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as handle:
        return yaml.safe_load(handle)


def validate(root: Path) -> tuple[list[str], list[str], dict[str, Any]]:
    errors: list[str] = []
    warnings: list[str] = []

    json_files = sorted(root.rglob("*.json"))
    yaml_files = sorted([*root.rglob("*.yaml"), *root.rglob("*.yml")])
    toml_files = sorted(root.rglob("*.toml"))

    for path in json_files:
        try:
            parsed = json.loads(path.read_text(encoding="utf-8"))
            if path.name.endswith(".schema.json"):
                jsonschema.Draft202012Validator.check_schema(parsed)
        except Exception as exc:  # noqa: BLE001 - aggregate diagnostics
            errors.append(f"JSON {path.relative_to(root)}: {exc}")

    for path in toml_files:
        try:
            tomllib.loads(path.read_text(encoding="utf-8"))
        except Exception as exc:  # noqa: BLE001 - aggregate diagnostics
            errors.append(f"TOML {path.relative_to(root)}: {exc}")

    yaml_objects: dict[Path, Any] = {}
    for path in yaml_files:
        try:
            yaml_objects[path] = load_yaml(path)
        except Exception as exc:  # noqa: BLE001 - aggregate diagnostics
            errors.append(f"YAML {path.relative_to(root)}: {exc}")

    schema_path = root / "schemas" / "work-item.schema.json"
    if not schema_path.exists():
        errors.append("Missing schemas/work-item.schema.json")
        return errors, warnings, {}

    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    validator = jsonschema.Draft202012Validator(schema)

    ids: dict[str, Path] = {}
    dependencies: dict[str, list[str]] = {}
    statuses: Counter[str] = Counter()
    kinds: Counter[str] = Counter()
    waves: Counter[str] = Counter()

    packet_paths = [
        path
        for path in yaml_files
        if path.is_relative_to(root / "work-items") and path.parent.name.startswith("wave-")
    ]

    for path in packet_paths:
        packet = yaml_objects.get(path)
        if not isinstance(packet, dict):
            errors.append(f"Work item {path.relative_to(root)} is not a YAML mapping")
            continue

        for issue in validator.iter_errors(packet):
            location = ".".join(str(part) for part in issue.path) or "<root>"
            errors.append(f"SCHEMA {path.relative_to(root)} at {location}: {issue.message}")

        work_id = packet.get("id")
        if not isinstance(work_id, str):
            continue
        if work_id in ids:
            errors.append(
                f"Duplicate work-item ID {work_id}: "
                f"{ids[work_id].relative_to(root)} and {path.relative_to(root)}"
            )
        ids[work_id] = path
        dependencies[work_id] = list(packet.get("depends_on") or [])
        statuses[str(packet.get("status"))] += 1
        kinds[str(packet.get("kind"))] += 1
        waves[path.parent.name] += 1

        expected_evidence = f".tdd/evidence/{work_id}"
        if packet.get("evidence_directory") != expected_evidence:
            errors.append(
                f"{work_id}: evidence_directory must be {expected_evidence!r}, "
                f"got {packet.get('evidence_directory')!r}"
            )

        for requirement_id in packet.get("requirement_ids") or []:
            prefix = str(requirement_id).split("-", 1)[0]
            if prefix not in ALLOWED_REQUIREMENT_PREFIXES:
                errors.append(f"{work_id}: unknown requirement prefix in {requirement_id}")

        if packet.get("kind") != "research":
            test = packet.get("test") or {}
            for key in ("red_command", "green_command"):
                command = test.get(key)
                if not isinstance(command, str) or not command.strip():
                    errors.append(f"{work_id}: missing or empty test.{key}")
                elif re.search(r"<[^>]+>", command):
                    warnings.append(f"{work_id}: test.{key} still contains a placeholder: {command}")

    for work_id, deps in dependencies.items():
        for dependency in deps:
            if dependency not in ids:
                errors.append(f"{work_id}: dependency {dependency} does not exist")

    indegree = {work_id: 0 for work_id in ids}
    outgoing: dict[str, list[str]] = defaultdict(list)
    for work_id, deps in dependencies.items():
        for dependency in deps:
            if dependency in ids:
                outgoing[dependency].append(work_id)
                indegree[work_id] += 1

    queue = deque(sorted(work_id for work_id, value in indegree.items() if value == 0))
    seen: list[str] = []
    while queue:
        current = queue.popleft()
        seen.append(current)
        for dependent in outgoing[current]:
            indegree[dependent] -= 1
            if indegree[dependent] == 0:
                queue.append(dependent)

    if len(seen) != len(ids):
        cyclic = sorted(work_id for work_id, value in indegree.items() if value > 0)
        errors.append(f"Dependency graph contains a cycle involving: {', '.join(cyclic)}")

    index_path = root / "work-items" / "index.yaml"
    if not index_path.exists():
        errors.append("Missing work-items/index.yaml")
    else:
        index = load_yaml(index_path)
        index_entries: dict[str, dict[str, Any]] = {}
        for wave in (index or {}).get("waves", []):
            for item in wave.get("items", []):
                work_id = item.get("id")
                if work_id in index_entries:
                    errors.append(f"Duplicate work-item index entry {work_id}")
                index_entries[work_id] = item
                relative_packet_path = item.get("path")
                packet_path = root / str(relative_packet_path)
                if not packet_path.exists():
                    errors.append(f"Index {work_id}: missing packet path {relative_packet_path}")
                    continue
                packet = load_yaml(packet_path)
                for key in ("id", "title", "status", "priority", "depends_on"):
                    if packet.get(key) != item.get(key):
                        errors.append(
                            f"Index mismatch {work_id}.{key}: "
                            f"packet={packet.get(key)!r}, index={item.get(key)!r}"
                        )

        for work_id in sorted(set(ids) - set(index_entries)):
            errors.append(f"Packet {work_id} is missing from work-items/index.yaml")
        for work_id in sorted(set(index_entries) - set(ids)):
            errors.append(f"Index entry {work_id} has no packet file")

    metrics = {
        "files": sum(1 for path in root.rglob("*") if path.is_file()),
        "work_items": len(ids),
        "statuses": dict(sorted(statuses.items())),
        "kinds": dict(sorted(kinds.items())),
        "waves": dict(sorted(waves.items())),
        "json_files": len(json_files),
        "yaml_files": len(yaml_files),
        "toml_files": len(toml_files),
    }
    return errors, warnings, metrics


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--root",
        type=Path,
        default=Path(__file__).resolve().parents[1],
        help="Topology starter-kit root (defaults to the parent of tools/)",
    )
    args = parser.parse_args()
    root = args.root.resolve()

    errors, warnings, metrics = validate(root)
    print(json.dumps(metrics, indent=2, sort_keys=True))
    for warning in warnings:
        print(f"WARNING: {warning}", file=sys.stderr)
    for error in errors:
        print(f"ERROR: {error}", file=sys.stderr)

    if errors:
        print(f"FAILED: {len(errors)} error(s), {len(warnings)} warning(s)", file=sys.stderr)
        return 1
    print(f"PASS: 0 errors, {len(warnings)} warning(s)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
