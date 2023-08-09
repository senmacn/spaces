#[derive(Debug, serde::Serialize)]
pub struct SpacesError {
    message: String,
}

impl SpacesError {
    pub fn new() -> SpacesError {
        SpacesError {
            message: String::from("内部错误"),
        }
    }

    pub fn from_str(message: &str) -> SpacesError {
        SpacesError {
            message: String::from(message),
        }
    }
}
