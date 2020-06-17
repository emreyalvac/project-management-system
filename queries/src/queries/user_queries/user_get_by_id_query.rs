use domain::query::query::TQuery;
use domain::user::user::User;

pub struct UserGetByIdQuery {
    pub id: String
}

impl TQuery<User> for UserGetByIdQuery {}
