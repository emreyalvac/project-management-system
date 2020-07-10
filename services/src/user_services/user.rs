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
use domain::user::validate_user::ValidateUserClaims;
use commands::commands::user_commands::validate_user_command::ValidateUserCommand;
use domain::query::query::TQueryHandler;
use domain::user::user_get_by_id::UserGetById;
use queries::queries::user_queries::get_user_boards_aggregate_query::GetUserBoardsAggregateQuery;
use domain::aggregates::board_user_aggregate::BoardUserAggregate;
use domain::user::insert_board_to_user::InsertBoardToUser;
use commands::commands::user_commands::insert_board_to_user_command::InsertBoardToUserCommand;
use crate::board_services::board::{BoardServices, TBoardServices};
use domain::board::insertable_board::InsertableBoard;
use queries::queries::user_queries::user_get_by_id_query::UserGetByIdQuery;
use domain::board::board_status::BoardStatus;
use domain::user::invite_user_to_board::InviteUserToBoard;
use domain::common::common_response::CommonResponse;
use domain::common::invite_user_claims::{InviteUserClaims, SubInviteUserClaims};
use domain::user::invite_user_response::InviteUserResponse;
use commands::commands::user_commands::check_and_apply_invite_command::CheckAndApplyInviteCommand;
use queries::queries::user_queries::check_user_board_query::CheckUserBoardQuery;
use domain::user::update_user::UpdateUser;
use commands::commands::user_commands::update_user_command::UpdateUserCommand;
use mongodb::Client;

#[async_trait]
pub trait TUserServices {
    async fn get_by_email(&self, user: GetByEmail) -> Result<User, NotFound>;
    async fn login(&self, login: LoginUser) -> Result<LoginResponse, LoginResponse>;
    async fn register(&self, mut register: Register) -> Result<CommandResponse, CommandResponse>;
    async fn generate_token_for_validation(&self, user: Register) -> String;
    async fn validate_user(&self, token: String) -> Result<CommandResponse, CommandResponse>;
    async fn get_user_boards(&self, user: UserGetById) -> Result<BoardUserAggregate, BoardUserAggregate>;
    async fn insert_board(&self, board: InsertBoardToUser) -> Result<CommandResponse, CommandResponse>;
    async fn get_by_id(&self, user: UserGetById) -> Result<User, NotFound>;
    async fn invite_user_with_email(&self, invite: InviteUserToBoard) -> Result<InviteUserResponse, CommonResponse>;
    async fn check_and_apply_invite(&self, token: String) -> Result<bool, bool>;
    async fn check_user_board(&self, board_id: String, user_id: String) -> Result<bool, bool>;
    async fn update_user(&self, user: UpdateUser) -> Result<CommandResponse, CommandResponse>;
}

pub struct UserServices {
    pub client: Client
}

static SECRET_KEY: &'static str = "d41d8cd98f00b204e9800998ecf8427e";

#[async_trait]
impl TUserServices for UserServices {
    async fn get_by_email(&self, user: GetByEmail) -> Result<User, NotFound> {
        let client = self.client.to_owned();
        let factory = UserQueryHandlerFactory { client };
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
                    let date: DateTime<Utc>;
                    if login.remember_me {
                        date = Utc::now() + Duration::days(365)
                    } else {
                        date = Utc::now() + Duration::hours(1)
                    }
                    let my_claims = Claims {
                        sub: user.id,
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
        let client = self.client.to_owned();
        match check_is_exist {
            Ok(_) => {
                Err(CommandResponse { message: "Found".to_owned(), command_type: CommandType::UserInsert, status: false })
            }
            Err(_) => {
                let factory = UserCommandHandlerFactory { client };
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
        let client = self.client.to_owned();
        let factory = UserCommandHandlerFactory { client };
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

    async fn get_user_boards(&self, user: UserGetById) -> Result<BoardUserAggregate, BoardUserAggregate> {
        let client = self.client.to_owned();
        let factory = UserQueryHandlerFactory { client };
        let handler = factory.build_for_boards(GetUserBoardsAggregateQuery { user_id: user.user_id }).await;
        let result = handler.get().await;
        match result {
            Ok(data) => Ok(data),
            Err(e) => Ok(e)
        }
    }

    async fn insert_board(&self, board: InsertBoardToUser) -> Result<CommandResponse, CommandResponse> {
        let client = self.client.to_owned();
        let client_clone = self.client.clone();
        let factory = UserCommandHandlerFactory { client };
        let board_service = BoardServices { client: client_clone };
        let cloned_board = board.clone();
        let insert_board = board.clone();
        let board = InsertableBoard {
            board_id: insert_board.board_id,
            board_cards: insert_board.board_cards,
            board_name: insert_board.board_name,
            board_manager_user_id: insert_board.user_id,
            board_status: BoardStatus::InProgress,
        };
        let _board_execute = board_service.insert_board(board).await;
        let mut handler = factory.build_for_insert_board(InsertBoardToUserCommand { user_board: cloned_board });
        let result = handler.execute().await;
        if result.status {
            Ok(result)
        } else {
            Err(result)
        }
    }

    async fn get_by_id(&self, user: UserGetById) -> Result<User, NotFound> {
        let client = self.client.to_owned();
        let factory = UserQueryHandlerFactory { client };
        let handler = factory.build_for_get_by_id(UserGetByIdQuery { id: user.user_id }).await;
        let result = handler.get().await;
        match result {
            Ok(user) => {
                Ok(user)
            }
            Err(err) => {
                Err(err)
            }
        }
    }

    async fn invite_user_with_email(&self, invite: InviteUserToBoard) -> Result<InviteUserResponse, CommonResponse> {
        let invite_user = invite;
        let board_id = invite_user.board_id.clone();
        let user_find = self.get_by_email(GetByEmail { email: invite_user.email }).await;
        match user_find {
            Ok(user) => {
                let ref_user = &user;
                if self.check_user_board(board_id, (&ref_user.id).parse().unwrap()).await.is_ok() {
                    let key = SECRET_KEY.as_bytes();
                    let date: DateTime<Utc> = Utc::now() + Duration::hours(24);
                    let my_claims = InviteUserClaims {
                        sub: SubInviteUserClaims { board_id: invite_user.board_id, user_id: (&ref_user.id).parse().unwrap() },
                        exp: date.timestamp() as usize,
                    };
                    match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key)) {
                        Ok(token) => {
                            return Ok(InviteUserResponse { email: user.email, token });
                        }
                        Err(_) => {
                            return Err(CommonResponse { status: false, message: "Token Generate Error".to_owned() });
                        }
                    };
                } else {
                    return Err(CommonResponse { status: false, message: "User already have this board".to_owned() });
                }
            }
            Err(_) => {
                return Err(CommonResponse { status: false, message: "User not found".to_owned() });
            }
        }
    }

    // TODO: Change bool response with DTO
    async fn check_and_apply_invite(&self, token: String) -> Result<bool, bool> {
        let key = SECRET_KEY.as_bytes();
        match decode::<InviteUserClaims>(token.as_str(), &DecodingKey::from_secret(key), &Validation::new(Algorithm::HS256)) {
            Ok(claims) => {
                let user = &claims.claims.sub;
                if self.check_user_board((&user.board_id).parse().unwrap(), (&user.user_id).parse().unwrap()).await.is_ok() {
                    let client = self.client.to_owned();
                    let factory = UserCommandHandlerFactory { client };
                    let mut handler = factory.build_for_check_and_apply_invite(CheckAndApplyInviteCommand { board_id: (&user.board_id).parse().unwrap(), user_id: (&user.user_id).parse().unwrap() });
                    let result = handler.execute().await;
                    if result.status {
                        return Ok(true);
                    } else {
                        return Err(false);
                    }
                } else {
                    return Err(false);
                }
            }
            Err(_) => {
                return Err(false);
            }
        }
    }

    async fn check_user_board(&self, board_id: String, user_id: String) -> Result<bool, bool> {
        let client = self.client.to_owned();
        let factory = UserQueryHandlerFactory { client };
        let handler = factory.build_for_check_user_board(CheckUserBoardQuery { board_id, user_id }).await;
        let result = handler.get().await;
        match result {
            Ok(_) => {
                Ok(true)
            }
            Err(_) => {
                Err(false)
            }
        }
    }

    async fn update_user(&self, user: UpdateUser) -> Result<CommandResponse, CommandResponse> {
        let client = self.client.to_owned();
        let factory = UserCommandHandlerFactory { client };
        let mut handler = factory.build_for_update(UpdateUserCommand { user });
        let result = handler.execute().await;
        if result.status {
            Ok(result)
        } else {
            Err(result)
        }
    }
}
