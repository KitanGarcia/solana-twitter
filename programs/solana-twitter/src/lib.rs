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

#[account] // Custom Rust attribute provided by anchor. Removes tons of boilerplate
pub struct Tweet { // Rust struct that defines properties (no methods) of our tweet
  pub author: Pubkey, // User who published this tweet
  pub timestamp: i64,
  pub topic: String, // Optional topic field that will appear on that topic's page
  pub content: String, // Actual content of the tweet
}
