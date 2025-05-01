pub use chrono::{NaiveDate, Utc};

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        FormError {
            form_values: (field_name, field_value),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            first_name,
            last_name,
            birth,
            birth_location,
            password,
        }
    }

    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
        let mut errors: Vec<&str> = Vec::new();
        if self.first_name.is_empty() {
            errors.push("No user name");
        }
        if self.password.len() < 8 {
            errors.push("At least 8 characters");
        }

        let mut has_alphabetic = false;
        let mut has_numeric = false;
        let mut has_none_alphanumeric = false;

        for c in self.password.chars() {
            if c.is_alphabetic() {
                has_alphabetic = true;
            } else if c.is_numeric() {
                has_numeric = true;
            } else if !c.is_whitespace() {
                has_none_alphanumeric = true;
            }
        }
        if self.password.len() >= 8 {
            if !(has_alphabetic && has_numeric && has_none_alphanumeric) {
                errors.push("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)");
            }
        }

        if errors.is_empty() {
            Ok(vec!["Valid first name", "Valid password"])
        } else {
            Err(FormError::new(
                if self.first_name.is_empty() {
                    String::from("first_name")
                } else {
                    String::from("password")
                },
                if self.first_name.is_empty() {
                    self.first_name.clone()
                } else {
                    self.password.clone()
                },
                errors.join(", "),
            ))
        }
    }
}
