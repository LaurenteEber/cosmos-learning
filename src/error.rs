// use cw_multi_test::error;
use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized{},

    #[error("Too many poll options")]
    TooManyOptions {},

    #[error("Poll not found")]
    PollNotFound {},

    #[error("Option dosn't found in the poll")]
    OptionNotFound {},

    // #[error("Custom Error val: {val:?}")]
    // CustomError { val: String },
}
