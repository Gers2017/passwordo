use serde::Serialize;
#[derive(Serialize)]
pub struct PasswordInfo {
    pub text: String,
    pub entropy: u32,
}

impl PasswordInfo {
    pub fn new(text: String, entropy: u32) -> Self {
        PasswordInfo { text, entropy }
    }
}
