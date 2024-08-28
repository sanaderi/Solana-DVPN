use anchor_lang::prelude::*;

declare_id!("4o5rQEqjUmKd8FAsUhHjgqze5goybGCyXZx9dS5wzg2M");

#[program]
pub mod dvpn_client {

    use super::*;

    // Define the 'create_plan' function
    pub fn create_plan(ctx: Context<CreatePlan>, expiration_date: i64) -> Result<()> {
        let plan = &mut ctx.accounts.plan;
        plan.owner = *ctx.accounts.user.key;
        plan.expiration_date = expiration_date;
        Ok(())
    }
}

// Define the context for 'create_plan'
#[derive(Accounts)]
pub struct CreatePlan<'info> {
    #[account(init, payer = user, space = 8 + 32 + 8)]
    // Space includes: discriminator + Pubkey + i64
    pub plan: Account<'info, Plan>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Define the 'Plan' account structure
#[account]
pub struct Plan {
    pub owner: Pubkey,
    pub expiration_date: i64,
}
