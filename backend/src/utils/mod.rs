// Utilities module stub
pub mod errors {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum AppError {
        #[error("not found")] 
        NotFound,
        #[error("unauthorized")] 
        Unauthorized,
        #[error("bad request: {0}")] 
        BadRequest(String),
        #[error(transparent)]
        Anyhow(#[from] anyhow::Error),
    }
}
