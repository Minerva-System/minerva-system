#[derive(Debug)]
pub struct EnumError {
    pub message: String,
    pub status: u16,
}

impl EnumError {
    pub fn precondition_failed(message: String) -> Self {
        Self {
            message,
            status: 412,
        }
    }
}
