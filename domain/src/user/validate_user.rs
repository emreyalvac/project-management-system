use serde::{Serialize, Deserialize};
use crate::user::register::Register;

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidateUserClaims {
    pub exp: usize,
    pub request_user: Register,
}
