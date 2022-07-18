use std::env::VarError;

#[derive(Debug)]
pub enum Error {
    EnvError(VarError),
    ApiError(String),
    SerdeError(String),
}
