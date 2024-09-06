use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

declare_id!("6ep5FBgrzidkqkqboodgHfYGVasqB6Z3BzF8uQQAcyjC");

pub const MAXIMUM_AGE: u64 = 60; // One minute
pub const FEED_ID: &str = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d"; // SOL/USD price feed id from https://pyth.network/developers/price-feed-ids

#[program]
pub mod dvpn_client {

    use super::*;

    // Define the 'create_plan' function
    pub fn create_plan(
        ctx: Context<CreatePlan>,
        title: String,
        expiration_date: i64,
    ) -> Result<()> {
        let plan = &mut ctx.accounts.plan;
        // plan.title = title;
        msg!("program called");
        let price_update = &mut ctx.accounts.price_update;
        let price = price_update.get_price_no_older_than(
            &Clock::get()?,
            MAXIMUM_AGE,
            &get_feed_id_from_hex(FEED_ID)?,
        )?;
        let price_string = price.price.to_string(); // Assuming `price` has a `price` field

        plan.title = title;
        plan.title = price_string;

        plan.owner = *ctx.accounts.user.key;
        // plan.title = title;
        plan.expiration_date = expiration_date;
        Ok(())
    }
}

// Define the context for 'create_plan'
#[derive(Accounts)]
pub struct CreatePlan<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 64)]
    // Space includes: discriminator + Pubkey + i64 + 64 bytes: For a fixed-length string field
    pub plan: Account<'info, Plan>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub price_update: Account<'info, PriceUpdateV2>,
}

// #[account(address = Pubkey::from_str(FEED_ID).unwrap() @ FeedError::InvalidPriceFeed)]

// Define the 'Plan' account structure
#[account]
pub struct Plan {
    pub owner: Pubkey,
    pub title: String,
    pub expiration_date: i64,
}

// #[error_code]
// pub enum FeedError {
//     #[msg("Invalid Price Feed")]
//     InvalidPriceFeed,
// }
