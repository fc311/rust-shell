use std::env;

pub fn parse_input(input: &str) -> Result<(String, Vec<String>), String> {
    let input = input.trim();
    let mut chars = input.chars().peekable();
    let mut command = String::new();
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_single_quotes = false;
    let mut in_double_quotes = false;
    let mut is_first_token = true;
    let mut in_escape = false;

    while let Some(c) = chars.next() {
        if in_escape {
            if in_double_quotes {
                match c {
                    '\\' | '$' | '"' | 'n' => {
                        current_arg.push(if c == 'n' { '\n' } else { c });
                        in_escape = false;
                    }
                    _ => return Err("parse error: invalid escape sequence".to_string()),
                }
            } else {
                current_arg.push(c);
                in_escape = false;
            }
            continue;
        }

        if !in_single_quotes && !in_double_quotes {
            match c {
                '\'' => {
                    in_single_quotes = true;
                }
                '"' => {
                    in_double_quotes = true;
                }
                ' ' => {
                    if !current_arg.is_empty() {
                        if is_first_token {
                            command = current_arg;
                            is_first_token = false;
                        } else {
                            args.push(current_arg);
                        }
                        current_arg = String::new();
                    }
                }
                '\\' => {
                    in_escape = true;
                }
                '$' if chars
                    .peek()
                    .map(|&c| c.is_alphabetic() || c == '_')
                    .unwrap_or(false) =>
                {
                    let mut var_name = String::new();
                    while let Some(&next_c) = chars.peek() {
                        if next_c.is_alphanumeric() || next_c == '_' {
                            var_name.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    let var_value = env::var(&var_name).unwrap_or_default();
                    current_arg.push_str(&var_value);
                }
                _ => current_arg.push(c),
            }
        } else if in_single_quotes {
            if c == '\'' {
                in_single_quotes = false;
                // Handle adjacent single quotes
                if chars.peek() == Some(&'\'') {
                    chars.next(); // Consume the second quote
                    in_single_quotes = true; // Stay in quote mode
                }
            } else {
                current_arg.push(c);
            }
        } else if in_double_quotes {
            match c {
                '"' => {
                    in_double_quotes = false;
                }
                '\\' => {
                    in_escape = true;
                }
                '$' if chars
                    .peek()
                    .map(|&c| c.is_alphabetic() || c == '_')
                    .unwrap_or(false) =>
                {
                    let mut var_name = String::new();
                    while let Some(&next_c) = chars.peek() {
                        if next_c.is_alphanumeric() || next_c == '_' {
                            var_name.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    let var_value = env::var(&var_name).unwrap_or_default();
                    current_arg.push_str(&var_value);
                }
                _ => current_arg.push(c),
            }
        }
    }

    if in_single_quotes {
        return Err("parse error: unclosed single quote".to_string());
    }
    if in_double_quotes {
        return Err("parse error: unclosed double quote".to_string());
    }
    if in_escape {
        return Err("parse error: incomplete escape sequence".to_string());
    }

    if !current_arg.is_empty() {
        if is_first_token {
            command = current_arg;
        } else {
            args.push(current_arg);
        }
    }

    if command.is_empty() {
        return Ok((String::new(), Vec::new()));
    }

    Ok((command, args))
}
