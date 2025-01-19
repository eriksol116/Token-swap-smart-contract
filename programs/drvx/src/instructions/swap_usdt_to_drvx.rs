use crate::{consts::GLOBAL_SEED, errors::CustomError, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct SwapUsdtToDrvx<'info> {
    #[account(
        mut,
        seeds = [GLOBAL_SEED.as_bytes()],
        bump
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    #[account(mut)]
    pub usdt_mint: Account<'info, Mint>,

    #[account(mut)]
    pub drvx_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = usdt_mint,
        associated_token::authority = global_state
    )]
    pub usdt_vault: Account<'info, TokenAccount>, // Pool's token account

    #[account(
        mut,
        associated_token::mint = drvx_mint,
        associated_token::authority = global_state
    )]
    pub drvx_vault: Account<'info, TokenAccount>, // Pool's token account

    #[account(
        mut,
        constraint = admin.key() == global_state.admin.key()
    )]
    pub admin: Signer<'info>,

    #[account(mut)]
    /// CHECK: fee wallet is safe since it is extra wallet
    pub fee_wallet: UncheckedAccount<'info>,

    #[account(
        mut,
        associated_token::mint = usdt_mint,
        associated_token::authority = fee_wallet
    )]
    pub fee_wallet_ata: Box<Account<'info, TokenAccount>>,

    // #[account(
    //     mut,
    //     associated_token::mint = usdt_mint,
    //     associated_token::authority = global_state,
    // )]
    // pub admin_usdt_token_account: Account<'info, TokenAccount>, // Admin's token account
    #[account(mut)]
    pub user: Signer<'info>,

    // #[account(
    //     mut,
    //     associated_token::mint = drvx_mint,
    //     associated_token::authority = global_state,
    // )]
    // pub admin_drvx_token_account: Account<'info, TokenAccount>, // Admin's token account
    #[account(
        mut,
        associated_token::mint = usdt_mint,
        associated_token::authority = user,
    )]
    pub user_usdt_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = drvx_mint,
        associated_token::authority = user,
    )]
    pub user_drvx_token_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>, // SPL token program
    pub system_program: Program<'info, System>,
}

pub fn swap_usdt_to_drvx(ctx: Context<SwapUsdtToDrvx>, amount: u64) -> Result<()> {
    let global_state = &mut ctx.accounts.global_state;
    let seeds: &[&[&[u8]]] = &[&[GLOBAL_SEED.as_bytes(), &[ctx.bumps.global_state]]];

    let user_usdt_token_account = &ctx.accounts.user_usdt_token_account;

    require!(
        user_usdt_token_account.amount >= amount,
        CustomError::InsufficientFundsInUserUsdtTokenAccount
    );

    require!(
        global_state.total_usdt_token_amount
            >= (amount as u64) * (global_state.swap_fee_rate as u64) / 10000,
        CustomError::InsufficientUsdtTokensInPool
    );

    require!(
        global_state.total_drvx_token_amount
            >= ((amount as u64) * (10000 - global_state.swap_fee_rate as u64)) / 10000,
        CustomError::InsufficientDrvxTokensInPool
    );

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new_with_signer(
        cpi_program,
        Transfer {
            from: ctx.accounts.user_usdt_token_account.to_account_info(),
            to: ctx.accounts.usdt_vault.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
        seeds,
    );
    token::transfer(cpi_context, amount)?;

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new_with_signer(
        cpi_program,
        Transfer {
            from: ctx.accounts.usdt_vault.to_account_info(),
            to: ctx.accounts.fee_wallet_ata.to_account_info(),
            authority: global_state.to_account_info(),
        },
        seeds,
    );
    token::transfer(
        cpi_context,
        (amount as u64) * (global_state.swap_fee_rate as u64) / 10000,
    )?;

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new_with_signer(
        cpi_program,
        Transfer {
            from: ctx.accounts.drvx_vault.to_account_info(),
            to: ctx.accounts.user_drvx_token_account.to_account_info(),
            authority: global_state.to_account_info(),
        },
        seeds,
    );
    token::transfer(
        cpi_context,
        amount * ((amount as u64) * (10000 - global_state.swap_fee_rate as u64)) / 10000,
    )?;

    global_state.total_usdt_token_amount +=
        amount * ((amount as u64) * (10000 - global_state.swap_fee_rate as u64)) / 10000;
    global_state.total_drvx_token_amount -=
        amount * ((amount as u64) * (10000 - global_state.swap_fee_rate as u64)) / 10000;
    // Reduce the global_state's token balance after the transfer

    Ok(())
}
