use chrono::Utc::*;

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        Self {
            form_values: (field_name, field_value),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name == "" {
            return Err(FormError {
                form_values: ("name", self.name.clone()),
                err: "Username is empty",
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            });
        }

        if self.password.chars().count() < 8 {
            return Err(FormError {
                form_values: ("password", self.password.clone()),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "Password should be at least 8 characters long",
            });
        }

        let mut is_letter = false;
        let mut is_digit = false;
        let mut is_symbol = false;

        for char in self.password.clone().chars().into_iter() {
            if char.is_digit(10) {
                is_digit = true;
            }

            if char.is_alphabetic() {
                is_letter = true;
            }

            if !char.is_alphanumeric() && !char.is_whitespace() {
                is_symbol = true;
            }
        }

        if !is_digit || !is_letter || !is_symbol {
            return Err(FormError {
                form_values: ("password", self.password.clone()),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "Password should be a combination of ASCII numbers, letters and symbols",
            });
        }

        Ok(())
    }
}
