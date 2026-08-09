//! Fixture provenance validation.

/// A deterministic, machine-readable fixture validation diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    /// Stable error category used by callers and CI diagnostics.
    pub code: String,
    /// YAML field path associated with the diagnostic.
    pub path: String,
    /// Human-readable context for local debugging.
    pub message: String,
}

impl ValidationError {
    fn new(code: &str, path: &str, message: impl Into<String>) -> Self {
        Self {
            code: code.to_owned(),
            path: path.to_owned(),
            message: message.into(),
        }
    }

    fn redistribution_permission(path: &str, message: impl Into<String>) -> Self {
        Self::new("redistribution_permission_required", path, message)
    }

    fn missing_field(path: &str) -> Self {
        Self::new(
            "missing_field",
            path,
            format!("required fixture provenance field `{path}` is missing"),
        )
    }

    fn invalid_field(path: &str, message: impl Into<String>) -> Self {
        Self::new("invalid_field", path, message)
    }

    fn duplicate_field(path: &str) -> Self {
        Self::new(
            "duplicate_field",
            path,
            format!("fixture provenance field `{path}` appears more than once"),
        )
    }
}

#[derive(Debug, Default)]
struct RedistributionFields {
    permitted: Option<bool>,
    basis: Option<String>,
}

/// Validate the redistribution declaration in a fixture provenance YAML record.
///
/// This validator deliberately owns only the redistribution permission rule. It
/// does not inspect fixture bytes, interpret legal language, or infer permission
/// from the source category.
pub fn validate_yaml(input: &str) -> Result<(), Vec<ValidationError>> {
    let fields = parse_redistribution(input)?;
    let Some(fields) = fields else {
        return Err(vec![ValidationError::missing_field("redistribution")]);
    };

    let mut errors = Vec::new();
    match fields.permitted {
        Some(true) => {}
        Some(false) => errors.push(ValidationError::redistribution_permission(
            "redistribution.permitted",
            "fixture redistribution permission must be explicitly permitted",
        )),
        None => errors.push(ValidationError::redistribution_permission(
            "redistribution.permitted",
            "fixture redistribution permission must be explicitly declared",
        )),
    }

    if !has_nonempty_basis(fields.basis.as_deref()) {
        errors.push(ValidationError::redistribution_permission(
            "redistribution.basis",
            "fixture redistribution permission requires a non-empty basis",
        ));
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn parse_redistribution(input: &str) -> Result<Option<RedistributionFields>, Vec<ValidationError>> {
    let mut section: Option<(usize, RedistributionFields)> = None;
    let mut result = None;
    let mut seen_redistribution = false;

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
            return Err(vec![ValidationError::invalid_field(
                "<root>",
                format!("line {line_number}: tabs cannot indent fixture provenance"),
            )]);
        }

        if let Some((section_indent, _)) = section.as_ref() {
            if indent > *section_indent {
                let Some((_, fields)) = section.as_mut() else {
                    return Err(vec![ValidationError::invalid_field(
                        "redistribution",
                        format!("line {line_number}: redistribution section is unavailable"),
                    )]);
                };
                let Some((key, value)) = split_mapping_entry(trimmed) else {
                    return Err(vec![ValidationError::invalid_field(
                        "redistribution",
                        format!("line {line_number}: expected a mapping entry"),
                    )]);
                };
                parse_redistribution_field(fields, key, value, line_number)?;
                continue;
            }

            if let Some((_, completed)) = section.take() {
                result = Some(completed);
            }
        }

        if indent != 0 {
            continue;
        }

        let Some((key, value)) = split_mapping_entry(trimmed) else {
            return Err(vec![ValidationError::invalid_field(
                "<root>",
                format!("line {line_number}: expected a top-level mapping entry"),
            )]);
        };

        if key != "redistribution" {
            continue;
        }

        if seen_redistribution {
            return Err(vec![ValidationError::duplicate_field("redistribution")]);
        }
        seen_redistribution = true;

        let value = strip_unquoted_comment(value).trim();
        if value.is_empty() {
            section = Some((indent, RedistributionFields::default()));
        } else if value.starts_with('{') && value.ends_with('}') {
            let mut fields = RedistributionFields::default();
            parse_inline_mapping(&mut fields, &value[1..value.len() - 1], line_number)?;
            result = Some(fields);
        } else {
            return Err(vec![ValidationError::invalid_field(
                "redistribution",
                format!("line {line_number}: redistribution must be a mapping"),
            )]);
        }
    }

    if let Some((_, completed)) = section {
        result = Some(completed);
    }

    Ok(result)
}

fn parse_inline_mapping(
    fields: &mut RedistributionFields,
    value: &str,
    line_number: usize,
) -> Result<(), Vec<ValidationError>> {
    for entry in value.split(',') {
        let entry = entry.trim();
        if entry.is_empty() {
            continue;
        }
        let Some((key, value)) = split_mapping_entry(entry) else {
            return Err(vec![ValidationError::invalid_field(
                "redistribution",
                format!("line {line_number}: expected an inline mapping entry"),
            )]);
        };
        parse_redistribution_field(fields, key, value, line_number)?;
    }
    Ok(())
}

fn parse_redistribution_field(
    fields: &mut RedistributionFields,
    key: &str,
    value: &str,
    line_number: usize,
) -> Result<(), Vec<ValidationError>> {
    let value = strip_unquoted_comment(value).trim();
    match key {
        "permitted" => {
            if fields.permitted.is_some() {
                return Err(vec![ValidationError::duplicate_field(
                    "redistribution.permitted",
                )]);
            }
            let parsed = match value {
                "true" | "True" | "TRUE" => true,
                "false" | "False" | "FALSE" => false,
                _ => {
                    return Err(vec![ValidationError::invalid_field(
                        "redistribution.permitted",
                        format!("line {line_number}: redistribution.permitted must be a boolean"),
                    )]);
                }
            };
            fields.permitted = Some(parsed);
        }
        "basis" => {
            if fields.basis.is_some() {
                return Err(vec![ValidationError::duplicate_field(
                    "redistribution.basis",
                )]);
            }
            fields.basis = Some(unquote_scalar(value).to_owned());
        }
        _ => {}
    }
    Ok(())
}

fn has_nonempty_basis(value: Option<&str>) -> bool {
    let Some(value) = value else {
        return false;
    };
    !matches!(
        value.trim(),
        "" | "null" | "Null" | "NULL" | "~" | "|" | ">"
    )
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
