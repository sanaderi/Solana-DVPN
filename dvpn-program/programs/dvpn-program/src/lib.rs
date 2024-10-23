use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use pyth_solana_receiver_sdk::price_update::{get_feed_id_from_hex, PriceUpdateV2};

declare_id!("EQHMWmJRCK9uNSXgn2sSVf6Q6Ruq2nhJYf2MgxD9jTBW");

pub const MAXIMUM_AGE: u64 = 60; // One minute
pub const FEED_ID: &str = "0xef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d"; // SOL/USD price feed id from https://pyth.network/developers/price-feed-ids

#[program]
pub mod dvpn_program {

    use super::*;

    // Define the 'create_plan' function
    pub fn create_plan(
        ctx: Context<CreatePlan>,
        expiration_date: i64,
        username: String,
    ) -> Result<()> {
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
        let system_program: &Program<'_, System> = &ctx.accounts.system_program;
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
        let server = &mut ctx.accounts.server;

        plan.owner = *ctx.accounts.user.key;
        let clock = Clock::get()?;
        let expire_timestamp = clock.unix_timestamp + expire_duration as i64;
        plan.expiration_date = expire_timestamp;
        plan.start_date = clock.unix_timestamp;
        plan.server = server.key();
        plan.paid_price = fund_lamports;
        plan.username = username;

        server.start_date = clock.unix_timestamp;
        server.waiting_fund += fund_lamports;
        if expire_timestamp > server.last_client_expiry {
            server.last_client_expiry = expire_timestamp;
        }
        server.client_count += 1;

        Ok(())
    }

    // Define the 'create_server' function
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
        server.client_count = 0;
        server.start_date = 0;
        server.waiting_fund = 0;
        server.last_client_expiry = 0;

        Ok(())
    }

    //For server claim income
    pub fn claim_income(ctx: Context<ClaimIncome>) -> Result<()> {
        let pda = &mut ctx.accounts.pda_account;
        let usr = &mut ctx.accounts.user.to_account_info();
        let server = &mut ctx.accounts.server;

        let clock = Clock::get()?;
        let total_time = server.last_client_expiry as u64 - server.start_date as u64;
        let total_time_past = clock.unix_timestamp as u64 - server.start_date as u64;

        // let system_program: &Program<'_, System> = &ctx.accounts.system_program;
        let fund_lamports: u64 = (total_time_past * server.waiting_fund / total_time) as u64;
        // let pda_balance_before = pda.get_lamports();

        if server.waiting_fund >= fund_lamports {
            server.waiting_fund -= fund_lamports as u64;
            server.start_date = clock.unix_timestamp;

            let signer: &[&[&[u8]]] = &[&[b"payment", &[255]]]; // Correctly include the bump

            let cpi_context = CpiContext::new_with_signer(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: pda.to_account_info(),
                    to: usr.clone(),
                },
                signer,
            );

            transfer(cpi_context, fund_lamports)?;
        }

        // let pda_balance_after = pda.get_lamports();

        Ok(())
    }
}

// Define the context for 'create_plan'
#[derive(Accounts)]
pub struct CreatePlan<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8 + 64 + 12)]
    // Space includes: discriminator + Pubkey + i64 + 64 bytes + username: For a fixed-length string field
    pub plan: Account<'info, Plan>,
    #[account(mut)] // Ensure the correct owner is updating the server
    pub server: Account<'info, Server>,
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
    pub server: Pubkey,
    pub expiration_date: i64,
    pub start_date: i64,
    pub paid_price: u64,
    pub username: String,
}

#[derive(Accounts)]
pub struct CreateServer<'info> {
    #[account(init,payer=user, space= 8 + 32 + 19 + 9 +10 + 8 + 8 + 8 + 8 + 8)]
    // Space includes: discriminator + owner + ip address string + portNum string + connectionType string + up_time + last_update
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
    pub client_count: i64,
    pub last_client_expiry: i64,
    pub start_date: i64,
    pub waiting_fund: u64,
}

#[derive(Accounts)]
pub struct ClaimIncome<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub server: Account<'info, Server>,
    #[account(
        mut,
        seeds = [b"payment".as_ref()],
        bump
    )]
    pub pda_account: SystemAccount<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The expiration date is less than 10 days.")]
    ExpirationTooSoon,

    #[msg("Invalid owner provided.")]
    InvalidOwner,
}
