use anchor_lang::prelude::*;

declare_id!("2yMtc57KbBGtMEyubqc75Dr21s5UdgmwYnYRTLosfv9t");

#[program]
pub mod solana_dvpn {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
