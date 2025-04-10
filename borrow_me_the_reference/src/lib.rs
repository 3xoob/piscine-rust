pub fn delete_and_backspace(s: &mut String) {
    let mut stack = Vec::new();

    for ch in s.chars() {
        match ch {
            '-' | '+' => {
                stack.pop();
            }
            _ => {
                stack.push(ch);
            }
        }
    }

    *s = stack.into_iter().collect();
}

pub fn do_operations(v: &mut [String]) {
    for expr in v {
        if let Some(idx) = expr.find('+') {
            let left = expr[..idx].trim().parse::<i32>().unwrap();
            let right = expr[idx + 1..].trim().parse::<i32>().unwrap();
            *expr = (left + right).to_string();
        } else if let Some(idx) = expr.find('-') {
            let left = expr[..idx].trim().parse::<i32>().unwrap();
            let right = expr[idx + 1..].trim().parse::<i32>().unwrap();
            *expr = (left - right).to_string();
        }
    }
}
