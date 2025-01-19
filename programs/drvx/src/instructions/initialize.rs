use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{consts::GLOBAL_SEED, errors::CustomError, state::*};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        space = 8 + std::mem::size_of::<GlobalState>(),
        payer = admin,
        seeds = [GLOBAL_SEED.as_bytes()],
        bump,
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    pub usdt_mint: Box<Account<'info, Mint>>,

    pub drvx_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = usdt_mint,
        associated_token::authority = global_state
    )]
    pub usdt_ata: Box<Account<'info, TokenAccount>>,

    #[account(
        init,
        payer = admin,
        associated_token::mint = drvx_mint,
        associated_token::authority = global_state
    )]
    pub drvx_ata: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub admin: Signer<'info>,

    /// CHECK:
    #[account(mut)]
    pub fee_wallet: UncheckedAccount<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let global_state = &mut ctx.accounts.global_state;

    if global_state.initialized {
        return err!(CustomError::AlreadyInitialized);
    }

    global_state.admin = ctx.accounts.admin.key();
    global_state.usdt_mint = ctx.accounts.usdt_mint.key();
    global_state.drvx_mint = ctx.accounts.drvx_mint.key();
    global_state.fee_wallet = ctx.accounts.fee_wallet.key();
    global_state.total_usdt_token_amount = 0;
    global_state.total_drvx_token_amount = 0;
    global_state.total_fee = 0;
    global_state.swap_count = 0;
    global_state.swap_rate = 1;
    global_state.swap_fee_rate = 250;
    global_state.total_swapped_usdt_tokens = 0;
    global_state.total_swapped_drvx_tokens = 0;
    global_state.initialized = true;

    Ok(())
}
