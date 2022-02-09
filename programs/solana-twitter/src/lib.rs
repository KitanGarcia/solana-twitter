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

// Define the structure of the Tweet account
#[account] // Custom Rust attribute provided by anchor. Removes tons of boilerplate
pub struct Tweet { // Rust struct that defines properties (no methods) of our tweet
  pub author: Pubkey, // User who published this tweet
  pub timestamp: i64,
  pub topic: String, // Optional topic field that will appear on that topic's page
  pub content: String, // Actual content of the tweet
}

// Add some useful constants for sizing properties
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const MAX_TOPIC_LENGTH: usize = 50 * 4; // 50 chars max.
const MAX_CONTENT_LENGTH: usize = 280 * 4; // 280 chars max.

// Add a constant on the Tweet account that provides its total size.
impl Tweet {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Author
        + TIMESTAMP_LENGTH
        + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH // Topic
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH; // Content
}
