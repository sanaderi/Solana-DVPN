use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

declare_id!("6cGNiQMuBXeshM5RqXMpAdVSB1YrUN3cUZ8i5hvZrMN3");

pub const MAXIMUM_AGE: u64 = 60; // One minute
pub const FEED_ID: &str = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d"; // SOL/USD price feed id from https://pyth.network/developers/price-feed-ids

#[program]
pub mod dvpn_program {

    use super::*;

    // Define the 'create_plan' function
    pub fn create_plan(ctx: Context<CreatePlan>, expiration_date: i64) -> Result<()> {
        if expiration_date < 10 {
            return Err(ErrorCode::ExpirationTooSoon.into());
        }
        //Retrieve solana price
        let price_update = &mut ctx.accounts.price_update;
        let price = price_update.get_price_no_older_than(
            &Clock::get()?,
            MAXIMUM_AGE,
            &get_feed_id_from_hex(FEED_ID)?,
        )?;

        // Calculate the precision factor
        let base = 10_f64;
        let expo = price.exponent as f64;
        let recieve_price = price.price as f64;
        let price_in_dollars = recieve_price * base.powf(expo);

        let price_per_sec: f64 = 2.0 / price_in_dollars / 2592000_f64;
        let expire_duration = expiration_date as f64 * 86400_f64;
        let account_price: f64 = price_per_sec * expire_duration;
        let fund_lamports: u64 = (account_price * 1000000000_f64).round() as u64;
        let pda = &mut ctx.accounts.pda_account;
        let signer = &mut ctx.accounts.user;
        let system_program = &ctx.accounts.system_program;
        let pda_balance_before = pda.get_lamports();

        transfer(
            CpiContext::new(
                system_program.to_account_info(),
                Transfer {
                    from: signer.to_account_info(),
                    to: pda.to_account_info(),
                },
            ),
            fund_lamports,
        )?;
        let pda_balance_after = pda.get_lamports();

        require_eq!(pda_balance_after, pda_balance_before + fund_lamports);

        let plan: &mut Account<'_, _> = &mut ctx.accounts.plan;

        plan.owner = *ctx.accounts.user.key;
        let clock = Clock::get()?;
        let expire_timestamp = clock.unix_timestamp + expire_duration as i64;
        plan.expiration_date = expire_timestamp;

        Ok(())
    }

    pub fn create_server(
        ctx: Context<CreateServer>,
        ip_address: String,
        port_num: String,
        connection_type: String,
    ) -> Result<()> {
        let server = &mut ctx.accounts.server;
        server.owner = ctx.accounts.user.key();
        server.ip_address = ip_address.to_string();
        server.port_num = port_num.to_string();
        server.connection_type = connection_type.to_string();

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
    #[account(
        mut,
        seeds = [b"payment".as_ref()],
        bump
    )]
    pub pda_account: SystemAccount<'info>,
}

// Define the 'Plan' account structure
#[account]
pub struct Plan {
    pub owner: Pubkey,
    pub expiration_date: i64,
}

#[derive(Accounts)]
pub struct CreateServer<'info> {
    #[account(init,payer=user, space= 8 + 32 + 19 + 9 +10)]
    // Space includes: discriminator + owner + ip address string + portNum string + connectionType string
    pub server: Account<'info, Server>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Server {
    pub owner: Pubkey,
    pub ip_address: String,      //Encrypted ip address
    pub port_num: String,        //Encrypted port number
    pub connection_type: String, //Encrypted connection type
}

#[error_code]
pub enum ErrorCode {
    #[msg("The expiration date is less than 10 days.")]
    ExpirationTooSoon,
}
