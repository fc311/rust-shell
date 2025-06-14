pub fn parse_input(input: &str) -> Result<(String, Vec<String>), String> {
    let input = input.trim();
    let mut chars = input.chars().peekable();
    let mut command = String::new();
    let mut args = Vec::new();
    let mut current_arg = String::new();
    let mut in_quotes = false;
    let mut is_first_token = true;

    while let Some(c) = chars.next() {
        if !in_quotes {
            match c {
                '\'' => {
                    if chars.peek() == Some(&'\'') {
                        return Err(
                            "parse error: single quote within single-quoted string".to_string()
                        );
                    }
                    in_quotes = true;
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
                _ => current_arg.push(c),
            }
        } else {
            if c == '\'' {
                in_quotes = false;
            } else {
                current_arg.push(c);
            }
        }
    }

    if in_quotes {
        return Err("parse error: unclosed single quote".to_string());
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
