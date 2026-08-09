//! Work-item packet validation.

/// A deterministic, machine-readable validation diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// Stable error category used by callers and CI diagnostics.
    pub code: String,
    /// YAML field path associated with the diagnostic.
    pub path: String,
    /// Human-readable context for logs and local debugging.
    pub message: String,
}

impl ValidationError {
    fn new(code: &str, path: &str, message: String) -> Self {
        Self {
            code: code.to_owned(),
            path: path.to_owned(),
            message,
        }
    }

    fn missing_field(path: &str) -> Self {
        Self::new(
            "missing_field",
            path,
            format!("required work-item field `{path}` is missing"),
        )
    }

    fn invalid_yaml(line: usize, message: impl Into<String>) -> Self {
        Self::new(
            "invalid_yaml",
            "<root>",
            format!("line {line}: {}", message.into()),
        )
    }
}

/// Validate the minimum structural invariant needed before a work packet can
/// enter orchestration: it must be a YAML mapping containing a non-empty `id`.
///
/// This deliberately does not validate the remaining packet schema. Later
/// packets add those independent rules through this same entry-point boundary.
pub fn validate_yaml(input: &str) -> Result<(), Vec<ValidationError>> {
    let document = parse_top_level_mapping(input)?;

    let Some(id) = document.id.as_deref() else {
        return Err(vec![ValidationError::missing_field("id")]);
    };

    if is_missing_scalar(id) {
        return Err(vec![ValidationError::missing_field("id")]);
    }

    Ok(())
}

#[derive(Debug, Default)]
struct TopLevelDocument {
    id: Option<String>,
}

fn parse_top_level_mapping(input: &str) -> Result<TopLevelDocument, Vec<ValidationError>> {
    let mut document = TopLevelDocument::default();
    let mut block_scalar_indent = None;

    for (line_index, raw_line) in input.lines().enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.trim_end_matches('\r');
        let trimmed = line.trim();

        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed == "---"
            || trimmed == "..."
            || trimmed.starts_with('%')
        {
            continue;
        }

        let indent = line
            .chars()
            .take_while(|character| *character == ' ')
            .count();
        if line[..indent].contains('\t') {
            return Err(vec![ValidationError::invalid_yaml(
                line_number,
                "tabs cannot indent a work-item mapping",
            )]);
        }

        if let Some(required_indent) = block_scalar_indent {
            if indent > required_indent {
                continue;
            }
            block_scalar_indent = None;
        }

        if indent != 0 {
            continue;
        }

        if trimmed.starts_with('-') {
            return Err(vec![ValidationError::invalid_yaml(
                line_number,
                "work item must be a YAML mapping",
            )]);
        }

        let Some((key, value)) = split_mapping_entry(trimmed) else {
            return Err(vec![ValidationError::invalid_yaml(
                line_number,
                "expected a top-level `key: value` entry",
            )]);
        };

        if key == "id" {
            if document.id.is_some() {
                return Err(vec![ValidationError::new(
                    "duplicate_field",
                    "id",
                    format!("line {line_number}: duplicate top-level field `id`"),
                )]);
            }

            let value = strip_unquoted_comment(value).trim();
            if value == "|" || value == ">" || value.starts_with("|-") || value.starts_with(">-") {
                block_scalar_indent = Some(indent);
            }
            document.id = Some(unquote_scalar(value).to_owned());
        }
    }

    Ok(document)
}

fn split_mapping_entry(line: &str) -> Option<(&str, &str)> {
    let mut quote = None;
    let mut escaped = false;

    for (index, character) in line.char_indices() {
        match quote {
            Some('"') => {
                if escaped {
                    escaped = false;
                } else if character == '\\' {
                    escaped = true;
                } else if character == '"' {
                    quote = None;
                }
            }
            Some('\'') => {
                if character == '\'' {
                    quote = None;
                }
            }
            None => match character {
                '"' | '\'' => quote = Some(character),
                ':' => {
                    let key = line[..index].trim();
                    let value = &line[index + 1..];
                    return (!key.is_empty()).then_some((unquote_scalar(key), value));
                }
                _ => {}
            },
            Some(_) => unreachable!("quote state only stores YAML quote characters"),
        }
    }

    None
}

fn strip_unquoted_comment(value: &str) -> &str {
    let mut quote = None;
    let mut escaped = false;

    for (index, character) in value.char_indices() {
        match quote {
            Some('"') => {
                if escaped {
                    escaped = false;
                } else if character == '\\' {
                    escaped = true;
                } else if character == '"' {
                    quote = None;
                }
            }
            Some('\'') => {
                if character == '\'' {
                    quote = None;
                }
            }
            None => match character {
                '"' | '\'' => quote = Some(character),
                '#' => return &value[..index],
                _ => {}
            },
            Some(_) => unreachable!("quote state only stores YAML quote characters"),
        }
    }

    value
}

fn unquote_scalar(value: &str) -> &str {
    let value = value.trim();
    if value.len() >= 2
        && ((value.starts_with('"') && value.ends_with('"'))
            || (value.starts_with('\'') && value.ends_with('\'')))
    {
        &value[1..value.len() - 1]
    } else {
        value
    }
}

fn is_missing_scalar(value: &str) -> bool {
    matches!(value.trim(), "" | "null" | "Null" | "NULL" | "~")
}
