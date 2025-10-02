use crate::user::UserUnwrapped;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Message {
    pub sender: UserUnwrapped,
    pub receiver: String,
    pub text: String,
}
