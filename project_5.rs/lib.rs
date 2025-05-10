pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("DxoVDhghq19PNJ1uM23GZkm1w8P5czyw3g9QodRGHtxi");

#[program]
pub mod swap {
    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>, id: u64, token_a_offered_amount: u64) -> Result<()> {
        instructions::make_offer::send_offered_tokens_to_vault(&ctx, token_a_offered_amount)?;
        instructions::make_offer::save_offer(ctx, id, token_a_offered_amount)
    }

    pub fn take_offer(ctx: Context<TakeOffer>) -> Result<()> {
        instructions::take_offer::send_wanted_tokens(&ctx)?;
        instructions::take_offer::withdraw_and_close_vault(ctx)
    }
}
