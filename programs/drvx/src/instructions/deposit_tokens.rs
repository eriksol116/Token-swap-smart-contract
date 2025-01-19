use crate::{consts::GLOBAL_SEED, errors::CustomError, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct DepositUsdtTokens<'info> {
    #[account(
        mut,
        seeds = [GLOBAL_SEED.as_bytes()],
        bump
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    #[account(
        mut,
        constraint = admin.key() == global_state.admin.key()
    )]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub token_mint: Account<'info, Mint>,

    #[account(
        mut,
        constraint = admin_token_account.owner == admin.key() 
    )]
    pub admin_token_account: Account<'info, TokenAccount>, // Admin's usdt token account

    #[account(mut)]
    pub global_state_token_account: Account<'info, TokenAccount>, // global_state's usdt token account

    pub token_program: Program<'info, Token>, // SPL token program
}

pub fn deposit_tokens(ctx: Context<DepositUsdtTokens>, amount: u64) -> Result<()> {
    let global_state = &mut ctx.accounts.global_state;
    let admin_token_account = &ctx.accounts.admin_token_account;
    require!(
        admin_token_account.amount >= amount,
        CustomError::InsufficientFundsInAdminUsdtTokenAccount
    );

    // Transfer tokens from the admin's token account to the pool's token account
    let cpi_accounts = Transfer {
        from: ctx.accounts.admin_token_account.to_account_info(),
        to: ctx
            .accounts
            .global_state_token_account
            .to_account_info(),
        authority: ctx.accounts.admin.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_context, amount)?;
    if ctx.accounts.token_mint.key() == global_state.usdt_mint.key(){
        global_state.total_usdt_token_amount += amount;

    } else {
        global_state.total_drvx_token_amount += amount;

    }

    Ok(())
}
