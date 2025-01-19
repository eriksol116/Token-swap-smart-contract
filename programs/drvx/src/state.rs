use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GlobalState {
    pub admin: Pubkey,
    pub fee_wallet: Pubkey,
    pub usdt_mint: Pubkey,
    pub drvx_mint: Pubkey,
    pub total_usdt_token_amount: u64,
    pub total_drvx_token_amount: u64,
    pub total_fee: u64,
    pub swap_count: u64,
    pub total_swapped_usdt_tokens: u64,
    pub total_swapped_drvx_tokens: u64,
    pub global_state_usdt_token_account: Pubkey,
    pub global_state_drvx_token_account: Pubkey,
    pub initialized: bool,
    pub swap_rate: u64,
    pub swap_fee_rate: u32,
}
