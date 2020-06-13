use domain::common::not_found::NotFound;
use domain::user::user::User;
use domain::user::get_by_email::GetByEmail;
use queries::factory::user_query_handler_factory::{UserQueryHandlerFactory, TUserQueryHandlerFactory};
use queries::queries::user_queries::user_get_by_email_query::UserGetByEmailQuery;
use async_trait::async_trait;
use domain::user::login_user::LoginUser;
use domain::user::login_response::LoginResponse;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use chrono::{DateTime, Utc, Duration};
use domain::common::claims::Claims;
use domain::command::command::TCommandHandler;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use domain::user::register::Register;
use domain::common::command_response::CommandResponse;
use domain::common::command_type::CommandType;
use commands::factory::user_command_handler_factory::{UserCommandHandlerFactory, TUserCommandHandlerFactory};
use commands::commands::user_commands::insert_user_command::InsertUserCommand;
use uuid::Uuid;
use domain::user::validate_user::ValidateUserClaims;
use commands::commands::user_commands::validate_user_command::ValidateUserCommand;
use domain::query::query::TQueryHandler;

#[async_trait]
pub trait TUserServices {
    async fn get_by_email(&self, user: GetByEmail) -> Result<User, NotFound>;
    async fn login(&self, login: LoginUser) -> Result<LoginResponse, LoginResponse>;
    async fn register(&self, mut register: Register) -> Result<CommandResponse, CommandResponse>;
    async fn generate_token_for_validation(&self, user: Register) -> String;
    async fn validate_user(&self, token: String) -> Result<CommandResponse, CommandResponse>;
}

pub struct UserServices {}

static SECRET_KEY: &'static str = "d41d8cd98f00b204e9800998ecf8427e";

#[async_trait]
impl TUserServices for UserServices {
    async fn get_by_email(&self, user: GetByEmail) -> Result<User, NotFound> {
        let factory = UserQueryHandlerFactory {};
        let query = factory.build_for_email(UserGetByEmailQuery { email: user.email }).await;
        let result = query.get().await;
        match result {
            Ok(user) => {
                Ok(user)
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    async fn login(&self, login: LoginUser) -> Result<LoginResponse, LoginResponse> {
        let user = self.get_by_email(GetByEmail { email: login.email }).await;
        match user {
            Ok(user) => {
                let mut sha = Sha256::new();
                sha.input_str(login.password.as_str());
                if user.password == sha.result_str() {
                    let key = SECRET_KEY.as_bytes();
                    let mut date: DateTime<Utc>;
                    if login.remember_me {
                        date = Utc::now() + Duration::days(365)
                    } else {
                        date = Utc::now() + Duration::hours(1)
                    }
                    let my_claims = Claims {
                        sub: user.email,
                        exp: date.timestamp() as usize,
                    };
                    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key)).unwrap();
                    if !user.is_validate {
                        return Err(LoginResponse { status: false, token: Default::default(), message: "User found but could not be verified".to_owned() });
                    }
                    Ok(LoginResponse { token, status: true, message: "OK".to_owned() })
                } else {
                    Err(LoginResponse { status: false, token: Default::default(), message: "Check your information.".to_owned() })
                }
            }
            Err(_) => {
                Err(LoginResponse { status: false, token: Default::default(), message: "Check your information.".to_owned() })
            }
        }
    }

    async fn register(&self, mut register: Register) -> Result<CommandResponse, CommandResponse> {
        let cloned_register = register.clone();
        let check_is_exist = self.get_by_email(GetByEmail { email: cloned_register.email }).await;
        match check_is_exist {
            Ok(_) => {
                Err(CommandResponse { message: "Found".to_owned(), command_type: CommandType::UserInsert, status: false })
            }
            Err(_) => {
                let factory = UserCommandHandlerFactory {};
                let mut sha = Sha256::new();
                sha.input(register.password.as_ref());
                register.password = sha.result_str();
                let mut handler = factory.build_for_insert(InsertUserCommand { user: register });
                let execute = handler.execute().await;
                if execute.status {
                    Ok(execute)
                } else {
                    Err(execute)
                }
            }
        }
    }

    async fn generate_token_for_validation(&self, user: Register) -> String {
        let expire: DateTime<Utc> = Utc::now() + Duration::minutes(30);
        let validate_user_claim = ValidateUserClaims {
            exp: expire.timestamp() as usize,
            request_user: user,
        };
        let secret = SECRET_KEY.as_bytes();
        let token = encode(&Header::default(), &validate_user_claim, &EncodingKey::from_secret(secret)).unwrap();
        token
    }

    async fn validate_user(&self, token: String) -> Result<CommandResponse, CommandResponse> {
        let factory = UserCommandHandlerFactory {};
        let key = SECRET_KEY.as_bytes();
        let decoded = match decode::<ValidateUserClaims>(token.as_str(), &DecodingKey::from_secret(key), &Validation::new(Algorithm::HS256)) {
            Ok(decoded) => {
                decoded
            }
            Err(_) => {
                return Err(CommandResponse { status: false, command_type: CommandType::ValidateUser, message: "Token Expired".to_owned() });
            }
        };
        let mut handler = factory.build_for_validate(ValidateUserCommand { id: decoded.claims.request_user.id });
        let execute = handler.execute().await;
        if execute.status {
            Ok(execute)
        } else {
            Err(execute)
        }
    }
}