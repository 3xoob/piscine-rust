pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match (server, security_level) {
        (Ok(url), _) => match security_level {
            Security::UnexpectedUrl => panic!("{}", url),
            _ => url.to_string(),
        },
        (Err(err), Security::Unknown) => panic!(),
        (Err(_), Security::Message) => panic!("ERROR: program stops"),
        (Err(_), Security::Warning) => "WARNING: check the server".to_string(),
        (Err(err), Security::NotFound) => format!("Not found: {}", err),
        (Err(err), Security::UnexpectedUrl) => err.to_string(),
    }
}
