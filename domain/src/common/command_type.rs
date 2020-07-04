use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandType {
    BoardInsert,
    UserInsert,
    ValidateUser,
    InsertBoardToUser,
    InsertBoard,
    InsertCard,
    InsertCardToBoard,
    InsertTaskToCard,
    MoveTaskToAnotherCard,
    UpdateCard,
    UpdateTask,
    DeleteTask,
    CheckAndApplyInvite,
    UpdateUser,
    AssignTaskToUser,
    DeleteAssignedUserFromTask
}
