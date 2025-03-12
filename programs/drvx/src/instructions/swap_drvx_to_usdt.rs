use crate::{consts::GLOBAL_SEED, errors::CustomError, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct SwapDrvxToUsdt<'info> {
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

    // #[account(mut)]
    // /// CHECK: fee wallet is safe since it is extra wallet
    // pub fee_wallet: UncheckedAccount<'info>,

    // #[account(
    //     mut,
    //     associated_token::mint = usdt_mint,
    //     associated_token::authority = global_state,
    // )]
    // pub admin_usdt_token_account: Account<'info, TokenAccount>, // Admin's token account
    // #[account(
    //     mut,
    //     associated_token::mint = usdt_mint,
    //     associated_token::authority = fee_wallet
    // )]
    // pub fee_wallet_ata: Box<Account<'info, TokenAccount>>,
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

pub fn swap_drvx_to_usdt(ctx: Context<SwapDrvxToUsdt>, amount: u64) -> Result<()> {
    let global_state = &mut ctx.accounts.global_state;
    let seeds: &[&[&[u8]]] = &[&[GLOBAL_SEED.as_bytes(), &[ctx.bumps.global_state]]];
    let usdt_decimal = ctx.accounts.usdt_mint.decimals;
    let drvx_decimal = ctx.accounts.drvx_mint.decimals;

    let user_drvx_token_account = &ctx.accounts.user_drvx_token_account;
    let usdt_vault = &ctx.accounts.usdt_vault;

    msg!(
        "global state value : usdt bal {}, drvx bal {}",
        global_state.total_usdt_token_amount,
        global_state.total_drvx_token_amount
    );
    msg!(
        "real balance in user : usdt bal {}, drvx bal {}",
        ctx.accounts.user_usdt_token_account.amount,
        user_drvx_token_account.amount
    );
    msg!(
        "real balance in vault : usdt bal {}, drvx bal {}",
        usdt_vault.amount,
        ctx.accounts.drvx_vault.amount
    );
    msg!(
        "global amount in vault : usdt bal {}, drvx bal {}",
        global_state.total_usdt_token_amount,
        global_state.total_drvx_token_amount
    );
    require!(
        user_drvx_token_account.amount > amount,
        CustomError::InsufficientFundsInUserDrvxTokenAccount
    );

    // let fee: u64 = (amount as f64 / (10u32.pow(drvx_decimal as u32) as f64)
    //     * (10u32.pow(usdt_decimal as u32) as f64)
    //     * global_state.swap_fee_rate as f64
    //     / 10000f64) as u64;

    let amount_out = (amount as f64 / (10u32.pow(drvx_decimal as u32) as f64)
        * (10u32.pow(usdt_decimal as u32) as f64)) as u64;
    // - fee;

    require!(
        usdt_vault.amount > amount_out,
        CustomError::InsufficientUsdtTokensInPool
    );

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new_with_signer(
        cpi_program,
        Transfer {
            from: ctx.accounts.user_drvx_token_account.to_account_info(),
            to: ctx.accounts.drvx_vault.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        },
        seeds,
    );
    token::transfer(cpi_context, amount)?;

    // let cpi_program = ctx.accounts.token_program.to_account_info();
    // let cpi_context = CpiContext::new_with_signer(
    //     cpi_program,
    //     Transfer {
    //         from: ctx.accounts.usdt_vault.to_account_info(),
    //         to: ctx.accounts.fee_wallet_ata.to_account_info(),
    //         authority: global_state.to_account_info(),
    //     },
    //     seeds,
    // );
    // token::transfer(cpi_context, fee)?;

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new_with_signer(
        cpi_program,
        Transfer {
            from: ctx.accounts.usdt_vault.to_account_info(),
            to: ctx.accounts.user_usdt_token_account.to_account_info(),
            authority: global_state.to_account_info(),
        },
        seeds,
    );
    token::transfer(cpi_context, amount_out)?;

    global_state.total_usdt_token_amount -= amount_out;
    global_state.total_drvx_token_amount += amount;
    // Reduce the global_state's token balance after the transfer

    Ok(())
}
