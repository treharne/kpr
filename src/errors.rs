#[derive(Debug)]

// std::fmt::Error
pub struct KprError {
    message: String,
}

impl KprError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
    pub fn default() -> Self {
        Self {
            message: "Unknown error".to_string(),
        }
    }
}

impl std::fmt::Display for KprError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for KprError {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<std::io::Error> for KprError {
    fn from(e: std::io::Error) -> Self {
        Self::new(&e.to_string())
    }
}

impl From<std::fmt::Error> for KprError {
    fn from(e: std::fmt::Error) -> Self {
        Self::new(&e.to_string())
    }
}