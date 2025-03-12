use anchor_lang::prelude::*;

pub mod consts;
pub mod errors;
pub mod event;
pub mod instructions;
pub mod state;

use crate::instructions::*;

declare_id!("FYC1X9f23fCHYx6eLrei9ErsJY6jkaojqptxYaCHWFdK");

#[program]
pub mod drvx {
    use super::*;

    // Admin initializes contract (one time only)
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    pub fn deposit_tokens(ctx: Context<DepositUsdtTokens>, amount: u64) -> Result<()> {
        instructions::deposit_tokens(ctx, amount)
    }

    pub fn withdraw_tokens(ctx: Context<WithdrawUsdtTokens>, amount: u64) -> Result<()> {
        instructions::withdraw_tokens(ctx, amount)
    }
    pub fn swap_usdt_to_drvx(ctx: Context<SwapUsdtToDrvx>, amount: u64) -> Result<()> {
        instructions::swap_usdt_to_drvx(ctx, amount)
    }
    pub fn swap_drvx_to_usdt(ctx: Context<SwapDrvxToUsdt>, amount: u64) -> Result<()> {
        instructions::swap_drvx_to_usdt(ctx, amount)
    }
}
