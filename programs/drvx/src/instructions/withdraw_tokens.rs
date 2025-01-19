use crate::{consts::GLOBAL_SEED, errors::CustomError, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct WithdrawUsdtTokens<'info> {
    #[account(
        mut,
        seeds = [GLOBAL_SEED.as_bytes()],
        bump
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    #[account(mut)]
    pub token_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = admin.key() == global_state.admin.key()
    )]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub global_state_token_account: Account<'info, TokenAccount>, // Pool's token account

    #[account(
        mut,
        constraint = admin_token_account.owner == admin.key()
    )]
    pub admin_token_account: Account<'info, TokenAccount>, // Admin's token account

    pub token_program: Program<'info, Token>, // SPL token program
}

pub fn withdraw_tokens(ctx: Context<WithdrawUsdtTokens>, amount: u64) -> Result<()> {
    let global_state = &mut ctx.accounts.global_state;
    let seeds: &[&[&[u8]]] = &[&[GLOBAL_SEED.as_bytes(), &[ctx.bumps.global_state]]];

    // Check if the global_state has enough tokens
    if ctx.accounts.token_mint.key() == global_state.usdt_mint.key() {
        require!(
            global_state.total_usdt_token_amount >= amount,
            CustomError::InsufficientUsdtTokensInPool
        );
    } else {
        require!(
            global_state.total_drvx_token_amount >= amount,
            CustomError::InsufficientDrvxTokensInPool
        );
    }

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new_with_signer(
        cpi_program,
        Transfer {
            from: ctx.accounts.global_state_token_account.to_account_info(),
            to: ctx.accounts.admin_token_account.to_account_info(),
            authority: global_state.to_account_info(),
        },
        seeds,
    );
    token::transfer(cpi_context, amount)?;

    if ctx.accounts.token_mint.key() == global_state.usdt_mint.key() {
        global_state.total_usdt_token_amount -= amount;
    } else {
        global_state.total_drvx_token_amount -= amount;
    }
    // Reduce the global_state's token balance after the transfer

    Ok(())
}
