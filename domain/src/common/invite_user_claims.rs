use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SubInviteUserClaims {
    pub user_id: String,
    pub board_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InviteUserClaims {
    pub sub: SubInviteUserClaims,
    pub exp: usize,
}
