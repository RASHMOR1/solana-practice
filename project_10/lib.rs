use anchor_lang::prelude::*;
use instructions::*;

mod constants;
mod error;
mod instructions;
mod state;

declare_id!("3FqUkZbaxpQZaLhGVLgqTD2wd3cZvYhc6Zu9jFbyNpNo");

#[program]
pub mod lending {
    use super::*;

    pub fn init_bank(
        ctx: Context<InitBank>,
        liquidation_threshold: u64,
        max_ltv: u64,
    ) -> Result<()> {
        process_init_bank(ctx, liquidation_threshold, max_ltv)
    }

    pub fn init_user(ctx: Context<InitUser>, usdc_address: Pubkey) -> Result<()> {
        process_init_user(ctx, usdc_address)
    }
    pub fn deposit(ctx: Context<Deposits>, amount: u64) -> Result<()> {
        process_deposit(ctx, amount)
    }
    pub fn withdrawa(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        process_withdrawal(ctx, amount)
    }
    pub fn borrow(ctx: Context<Borrow>, amount: u64) -> Result<()> {
        process_borrow(ctx, amount)
    }
    pub fn repay(ctx: Context<Repay>, amount: u64) -> Result<()> {
        process_repay(ctx, amount)
    }
    pub fn liquidate(ctx: Context<Liquidate>) -> Result<()> {
        process_liquidate(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize {}
