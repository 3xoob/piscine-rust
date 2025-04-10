pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut chars = s.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '-' => {
                result.pop();
            }
            '+' => {
                chars.next();
            }
            _ => {
                result.push(ch);
            }
        }
    }

    *s = result;
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
