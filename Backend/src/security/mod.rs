use thiserror::Error as ThisError;

pub struct UserCtx {
    pub user_id: i64,
}

pub async fn utx_from_token(token: &str) -> Result<UserCtx, Error> {
    // Todo real validation using auth needed
    // currently just doing a i64 parsing
    // form here we will get the utx which we  will pass to the todos...
    match token.parse::<i64>() {
        Ok(user_id) => Ok(UserCtx { user_id }),
        Err(_) => Err(Error::InvalidToken(token.to_string())),
    }
}

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("InvalidToken {0}")]
    InvalidToken(String),
}
