use domain::user::user::User;
use domain::query::query::TQuery;

pub struct UserGetByEmailQuery {
    pub email: String
}

impl TQuery<User> for UserGetByEmailQuery {}
