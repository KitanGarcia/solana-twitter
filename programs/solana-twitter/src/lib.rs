use anchor_lang::prelude::*;

declare_id!("8NMfKrcjkHAJpacYTLbL6gPb9TFbta5JwbhLHDGay9cY");

#[program]
pub mod solana_twitter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
