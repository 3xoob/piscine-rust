pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut skip_next = false;

    for (i, c) in s.chars().enumerate() {
        if skip_next {
            skip_next = false;
            continue;
        }

        match c {
            '-' => {
                if !result.is_empty() {
                    result.pop();
                }
            }
            '+' => {
                skip_next = true;
            }
            _ => {
                result.push(c);
            }
        }
    }

    *s = result;
}

pub fn do_operations(v: &mut [String]) {
    for item in v.iter_mut() {
        let parts: Vec<&str> = item.split(|c| c == '+' || c == '-').collect();
        if parts.len() != 2 {
            continue;
        }

        let num1: i32 = parts[0].trim().parse().unwrap_or(0);
        let num2: i32 = parts[1].trim().parse().unwrap_or(0);

        let operation = item.chars().find(|&c| c == '+' || c == '-').unwrap();
        let result = match operation {
            '+' => num1 + num2,
            '-' => num1 - num2,
            _ => unreachable!(),
        };

        *item = result.to_string();
    }
}
