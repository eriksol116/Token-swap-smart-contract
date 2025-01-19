use anchor_lang::prelude::*;

pub use CustomError::*;

#[error_code]
pub enum CustomError {
    #[msg("AlreadyInitialized")]
    AlreadyInitialized,

    #[msg("The caller is not the pool admin.")]
    PoolAdminMismatch, // Error when the caller is not the admin

    #[msg("The pool does not have enough USDT tokens.")]
    InsufficientUsdtTokensInPool, // Error when the pool lacks Usdt tokens

    #[msg("The pool does not have enough USDT tokens.")]
    InsufficientDrvxTokensInPool, // Error when the pool lacks drvx tokens

    #[msg("The admin does not have enough USDT tokens in their account for this deposit.")]
    InsufficientFundsInAdminUsdtTokenAccount,

    #[msg("The admin does not have enough DRVX tokens in their account for this deposit.")]
    InsufficientFundsInAdminDrvxTokenAccount,

    #[msg("The mint of the deposited tokens does not match the expected mint.")]
    MintMismatch, // New error for mint mismatch

    #[msg("User wallet does not have enough USDT tokens.")]
    InsufficientFundsInUserUsdtTokenAccount,

    #[msg("User wallet does not have enough DRVX tokens.")]
    InsufficientFundsInUserDrvxTokenAccount,
}
