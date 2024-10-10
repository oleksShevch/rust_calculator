fn shunting_yard(expression: &str) -> Result<String, &'static str> {
    let mut output_queue = Vec::new();
    let mut operator_stack = Vec::new();
    let mut num_buf = String::new();
    let mut last_was_operator = true;

    let precedence = |op: char| -> i32 {
        match op {
            '+' | '-' => 1,
            '*' | '/' => 2,
            _ => -1,
        }
    };

    let is_operator = |c: char| -> bool {
        match c {
            '+' | '-' | '*' | '/' => true,
            _ => false,
        }
    };

    for c in expression.chars() {
        if c.is_digit(10) || c == '.' || (c == '-' && last_was_operator) {
            num_buf.push(c);
            last_was_operator = false;
        } else {
            if !num_buf.is_empty() {
                output_queue.push(num_buf.clone());
                num_buf.clear();
            }

            if is_operator(c) {
                while let Some(&top_op) = operator_stack.last() {
                    if precedence(top_op) >= precedence(c) {
                        output_queue.push(operator_stack.pop().unwrap().to_string());
                    } else {
                        break;
                    }
                }
                operator_stack.push(c);
                last_was_operator = true;
            }
        }
    }

    if !num_buf.is_empty() {
        output_queue.push(num_buf);
    }

    while let Some(op) = operator_stack.pop() {
        output_queue.push(op.to_string());
    }

    Ok(output_queue.join(" ")) // Join tokens with space to create a single string
}

fn round_to_nearest(value: f64, decimals: usize) -> f64 {
    let factor = 10f64.powi(decimals as i32);
    (value * factor).round() / factor
}


fn evaluate_postfix(expression: &str) -> Result<f64, &'static str> {
    let mut stack = Vec::new();

    for token in expression.split_whitespace() {
        match token {
            "+" => {
                let b = stack.pop().ok_or("Invalid expression")?;
                let a = stack.pop().ok_or("Invalid expression")?;
                stack.push(a + b);
            }
            "-" => {
                let b = stack.pop().ok_or("Invalid expression")?;
                let a = stack.pop().ok_or("Invalid expression")?;
                stack.push(a - b);
            }
            "*" => {
                let b = stack.pop().ok_or("Invalid expression")?;
                let a = stack.pop().ok_or("Invalid expression")?;
                stack.push(a * b);
            }
            "/" => {
                let b = stack.pop().ok_or("Invalid expression")?;
                let a = stack.pop().ok_or("Invalid expression")?;
                stack.push(a / b);
            }
            _ => {
                let number = token.parse::<f64>().map_err(|_| "Invalid number")?;
                stack.push(number);
            }
        }
    }

    if stack.len() == 1 {
        Ok(round_to_nearest(stack.pop().unwrap(), 10))
    } else {
        Err("Invalid expression")
    }
}




pub(crate) fn calculate(expr: &str) -> Result<f64, &'static str> {
    if expr.starts_with("pfx ") {
        let postfix_expr = expr.trim_start_matches("pfx ").trim();
        evaluate_postfix(postfix_expr)
    } else {
        let converted_expr = shunting_yard(expr)?;
        evaluate_postfix(&converted_expr)
    }
}
