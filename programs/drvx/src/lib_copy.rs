use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use std::mem::size_of;

pub const DECIMALS: u32 = 9;

pub const GLOBAL_SEED: &[u8] = b"GLOBAL_SEED";

pub const VAULT_SEED: &[u8] = b"VAULT_SEED";

declare_id!("BCBVVHScphRYjnRqZUa7dZwVZbMY1jJUTQH5WbUTj2PP");

#[program]
pub mod drvx {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;
        global_state.admin = ctx.accounts.admin.key();
        global_state.fee_wallet = ctx.accounts.fee_wallet.key();
        global_state.total_usdt_tokens = 0;
        global_state.total_drvx_tokens = 0;
        global_state.total_fee = 0;
        global_state.total_swap_count = 0;
        global_state.swap_rate = 0;
        global_state.total_swap_usdt_tokens = 0;
        global_state.total_swap_drvx_tokens = 0;
        Ok(())
    }

    // pub fn deposit_usdt_tokens(ctx: Context<DepositUsdtTokens>, amount: u64) -> Result<()> {
    //     let global_state = &mut ctx.accounts.global_state;
    //     let admin_usdt_token_account = &ctx.accounts.admin_usdt_token_account;
    //     require!(
    //         admin_usdt_token_account.amount >= amount,
    //         CustomError::InsufficientFundsInAdminUsdtTokenAccount
    //     );

    //     // Transfer tokens from the admin's token account to the pool's token account
    //     let cpi_accounts = Transfer {
    //         from: ctx.accounts.admin_usdt_token_account.to_account_info(),
    //         to: ctx
    //             .accounts
    //             .global_state_usdt_token_account
    //             .to_account_info(),
    //         authority: ctx.accounts.admin.to_account_info(),
    //     };
    //     let cpi_program = ctx.accounts.token_program.to_account_info();
    //     let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    //     token::transfer(cpi_context, amount)?;

    //     // Update the global_state's total token balance
    //     global_state.total_usdt_tokens += amount;

    //     Ok(())
    // }

    // pub fn withdraw_usdt_tokens(ctx: Context<WithdrawUsdtTokens>, amount: u64) -> Result<()> {
    //     let global_state = &mut ctx.accounts.global_state;
    //     let seeds: &[&[&[u8]]] = &[&[GLOBAL_SEED, &[ctx.bumps.global_state]]];

    //     // Check if the global_state has enough tokens
    //     require!(
    //         global_state.total_swap_usdt_tokens >= amount,
    //         CustomError::InsufficientUsdtTokensInPool
    //     );

    //     let cpi_program = ctx.accounts.token_program.to_account_info();
    //     let cpi_context = CpiContext::new_with_signer(
    //         cpi_program,
    //         Transfer {
    //             from: ctx
    //                 .accounts
    //                 .global_state_usdt_token_account
    //                 .to_account_info(),
    //             to: ctx.accounts.admin_usdt_token_account.to_account_info(),
    //             authority: global_state.to_account_info(),
    //         },
    //         seeds,
    //     );
    //     token::transfer(cpi_context, amount)?;

    //     // Reduce the global_state's token balance after the transfer
    //     global_state.total_usdt_tokens -= amount;

    //     Ok(())
    // }

    // pub fn deposit_drvx_tokens(ctx: Context<DepositDrvxTokens>, amount: u64) -> Result<()> {
    //     let global_state = &mut ctx.accounts.global_state;
    //     let admin_drvx_token_account = &ctx.accounts.admin_drvx_token_account;
    //     require!(
    //         admin_drvx_token_account.amount >= amount,
    //         CustomError::InsufficientFundsInAdminDrvxTokenAccount
    //     );

    //     // Transfer tokens from the admin's token account to the pool's token account
    //     let cpi_accounts = Transfer {
    //         from: ctx.accounts.admin_drvx_token_account.to_account_info(),
    //         to: ctx
    //             .accounts
    //             .global_state_drvx_token_account
    //             .to_account_info(),
    //         authority: ctx.accounts.admin.to_account_info(),
    //     };
    //     let cpi_program = ctx.accounts.token_program.to_account_info();
    //     let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    //     token::transfer(cpi_context, amount)?;

    //     // Update the global_state's total token balance
    //     global_state.total_drvx_tokens += amount;

    //     Ok(())
    // }

    // pub fn withdraw_drvx_tokens(ctx: Context<WithdrawDrvxTokens>, amount: u64) -> Result<()> {
    //     let global_state = &mut ctx.accounts.global_state;
    //     let seeds: &[&[&[u8]]] = &[&[GLOBAL_SEED, &[ctx.bumps.global_state]]];

    //     // Check if the global_state has enough tokens
    //     require!(
    //         global_state.total_drvx_tokens >= amount,
    //         CustomError::InsufficientDrvxTokensInPool
    //     );

    //     let cpi_program = ctx.accounts.token_program.to_account_info();
    //     let cpi_context = CpiContext::new_with_signer(
    //         cpi_program,
    //         Transfer {
    //             from: ctx
    //                 .accounts
    //                 .global_state_drvx_token_account
    //                 .to_account_info(),
    //             to: ctx.accounts.admin_drvx_token_account.to_account_info(),
    //             authority: global_state.to_account_info(),
    //         },
    //         seeds,
    //     );
    //     token::transfer(cpi_context, amount)?;

    //     // Reduce the global_state's token balance after the transfer
    //     global_state.total_drvx_tokens -= amount;

    //     Ok(())
    // }

    // pub fn swap_usdt_to_drvx(ctx: Context<SwapUsdtToDrvx>, amount: u64) -> Result<()> {
    //     let global_state = &mut ctx.accounts.global_state;
    //     let seeds: &[&[&[u8]]] = &[&[GLOBAL_SEED, &[ctx.bumps.global_state]]];
    //     let swap_rate = global_state.swap_rate;

    //     // Check if the global_state has enough drvx tokens
    //     require!(
    //         ctx.accounts.global_state.global_state_drvx_token_account.a
    //             >= amount * (100 - swap_rate) / 100,
    //         CustomError::InsufficientDrvxTokensInPool
    //     );

    //     // Check if the global_state has enough drvx tokens
    //     require!(
    //         global_state.total_drvx_tokens >= amount * (100 - swap_rate) / 100,
    //         CustomError::InsufficientDrvxTokensInPool
    //     );

    //     // Check if the global_state has enough usdt tokens
    //     require!(
    //         global_state.total_tokens >= amount,
    //         CustomError::InsufficientTokensInPool
    //     );

    //     let cpi_program = ctx.accounts.token_program.to_account_info();
    //     let cpi_context = CpiContext::new_with_signer(
    //         cpi_program,
    //         Transfer {
    //             from: ctx.accounts.global_state_token_account.to_account_info(),
    //             to: ctx.accounts.user_token_account.to_account_info(),
    //             authority: global_state.to_account_info(),
    //         },
    //         seeds,
    //     );
    //     token::transfer(cpi_context, amount)?;

    //     // Reduce the global_state's token balance after the transfer
    //     global_state.total_tokens -= amount;

    //     Ok(())
    // }

    // pub fn swap_drvx_to_usdt(ctx: Context<SwapUsdtToDrvx>, amount: u64) -> Result<()> {
    //     let global_state = &mut ctx.accounts.global_state;
    //     let seeds: &[&[&[u8]]] = &[&[GLOBAL_SEED, &[ctx.bumps.global_state]]];

    //     // Check if the global_state has enough tokens
    //     require!(
    //         global_state.total_tokens >= amount,
    //         CustomError::InsufficientTokensInPool
    //     );
    //     let cpi_program = ctx.accounts.token_program.to_account_info();
    //     let cpi_context = CpiContext::new_with_signer(
    //         cpi_program,
    //         Transfer {
    //             from: ctx.accounts.global_state_token_account.to_account_info(),
    //             to: ctx.accounts.user_token_account.to_account_info(),
    //             authority: global_state.to_account_info(),
    //         },
    //         seeds,
    //     );
    //     token::transfer(cpi_context, amount)?;

    //     // Reduce the global_state's token balance after the transfer
    //     global_state.total_tokens -= amount;

    //     Ok(())
    // }
}

#[derive(Accounts)]
#[instruction()]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [GLOBAL_SEED],
        bump,
        space = 8 + size_of::<GlobalState>(),
        payer = admin
    )]
    pub global_state: Box<Account<'info, GlobalState>>,

    pub usdt_mint: Account<'info, Mint>,

    pub drvx_mint: Account<'info, Mint>,

    #[account(
        init,
        payer = admin,

        
    )]
    pub usdt_ata: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = admin,

        
    )]
    pub drvx_ata: Account<'info, TokenAccount>,


    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(mut)]
    pub fee_wallet: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>, // <- Add token program here
    pub system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// pub struct DepositUsdtTokens<'info> {
//     #[account(
//         mut,
//         seeds = [GLOBAL_SEED],
//         bump
//     )]
//     pub global_state: Account<'info, GlobalState>,

//     #[account(
//         mut,
//         constraint = admin.key() == global_state.admin.key()
//     )]
//     pub admin: Signer<'info>,
//     #[account(mut)]
//     pub usdt_mint: Account<'info, Mint>,

//     #[account(
//         mut,
//         constraint = admin_usdt_token_account.owner == admin.key()
//     )]
//     pub admin_usdt_token_account: Account<'info, TokenAccount>, // Admin's usdt token account

//     #[account(mut)]
//     pub global_state_usdt_token_account: Account<'info, TokenAccount>, // global_state's usdt token account
//     pub token_program: Program<'info, Token>, // SPL token program
// }

// #[derive(Accounts)]
// pub struct WithdrawUsdtTokens<'info> {
//     #[account(mut, seeds = [GLOBAL_SEED], bump)]
//     pub global_state: Account<'info, GlobalState>,
//     #[account(mut)]
//     pub usdt_mint: Account<'info, Mint>,
//     #[account(
//         mut,
//         constraint = admin.key() == global_state.admin.key()
//     )]
//     pub admin: Signer<'info>,
//     #[account(mut)]
//     pub global_state_usdt_token_account: Account<'info, TokenAccount>, // Pool's token account

//     #[account(
//         mut,
//         constraint = admin_usdt_token_account.owner == admin.key()
//     )]
//     pub admin_usdt_token_account: Account<'info, TokenAccount>, // Admin's token account
//     pub token_program: Program<'info, Token>, // SPL token program
// }

// #[derive(Accounts)]
// pub struct DepositDrvxTokens<'info> {
//     #[account(
//         mut,
//         seeds = [GLOBAL_SEED],
//         bump
//     )]
//     pub global_state: Account<'info, GlobalState>,

//     #[account(
//         mut,
//         constraint = admin.key() == global_state.admin.key()
//     )]
//     pub admin: Signer<'info>,
//     #[account(mut)]
//     pub drvx_mint: Account<'info, Mint>,

//     #[account(
//         mut,
//         constraint = admin_drvx_token_account.owner == admin.key()
//     )]
//     pub admin_drvx_token_account: Account<'info, TokenAccount>, // Admin's drvx token account

//     #[account(mut)]
//     pub global_state_drvx_token_account: Account<'info, TokenAccount>, // global_state's drvx token account
//     pub token_program: Program<'info, Token>, // SPL token program
// }

// #[derive(Accounts)]
// pub struct WithdrawDrvxTokens<'info> {
//     #[account(mut, seeds = [GLOBAL_SEED], bump)]
//     pub global_state: Account<'info, GlobalState>,
//     #[account(mut)]
//     pub drvx_mint: Account<'info, Mint>,
//     #[account(
//         mut,
//         constraint = admin.key() == global_state.admin.key()
//     )]
//     pub admin: Signer<'info>,
//     #[account(mut)]
//     pub global_state_drvx_token_account: Account<'info, TokenAccount>, // Pool's token account

//     #[account(
//         mut,
//         constraint = admin_drvx_token_account.owner == admin.key()
//     )]
//     pub admin_drvx_token_account: Account<'info, TokenAccount>, // Admin's token account
//     pub token_program: Program<'info, Token>, // SPL token program
// }

// #[derive(Accounts)]
// pub struct SwapUsdtToDrvx<'info> {
//     #[account(mut, seeds = [GLOBAL_SEED], bump)]
//     pub global_state: Account<'info, GlobalState>,
//     #[account(mut)]
//     pub mint: Box<Account<'info, Mint>>,
//     #[account(mut)]
//     pub user: Signer<'info>,
//     #[account(mut)]
//     pub global_state_usdt_token_account: Account<'info, TokenAccount>, // Pool's usdt token account
//     #[account(mut)]
//     pub global_state_drvx_token_account: Account<'info, TokenAccount>, // Pool's drvx token account
//     #[account(mut)]
//     pub user_usdt_token_account: Account<'info, TokenAccount>, // User's usdt token account
//     #[account(mut)]
//     pub user_drvx_token_account: Account<'info, TokenAccount>, // User's drvx token account
//     pub token_program: Program<'info, Token>, // SPL token program
// }

// #[derive(Accounts)]
// pub struct SwapDrvxToUsdt<'info> {
//     #[account(mut, seeds = [GLOBAL_SEED], bump)]
//     pub global_state: Account<'info, GlobalState>,
//     #[account(mut)]
//     pub mint: Box<Account<'info, Mint>>,
//     #[account(mut)]
//     pub user: Signer<'info>,
//     #[account(mut)]
//     pub global_state_usdt_token_account: Account<'info, TokenAccount>, // Pool's usdt token account
//     #[account(mut)]
//     pub global_state_drvx_token_account: Account<'info, TokenAccount>, // Pool's drvx token account
//     #[account(mut)]
//     pub user_usdt_token_account: Account<'info, TokenAccount>, // User's usdt token account
//     #[account(mut)]
//     pub user_drvx_token_account: Account<'info, TokenAccount>, // User's drvx token account
//     pub token_program: Program<'info, Token>, // SPL token program
// }

#[account]
#[derive(Default)]
pub struct GlobalState {
    pub admin: Pubkey,
    pub fee_wallet: Pubkey,
    pub total_usdt_tokens: u64,
    pub total_drvx_tokens: u64,
    pub total_fee: u64,
    pub total_swap_count: u64,
    pub swap_rate: u64,
    pub total_swap_usdt_tokens: u64,
    pub total_swap_drvx_tokens: u64,
    pub global_state_usdt_token_account: Pubkey,
    pub global_state_drvx_token_account: Pubkey,
}

// Define custom errors
#[error_code]
pub enum CustomError {
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
}
