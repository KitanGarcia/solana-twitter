use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("8NMfKrcjkHAJpacYTLbL6gPb9TFbta5JwbhLHDGay9cY");

#[program]
pub mod solana_twitter {
  use super::*;
  pub fn send_tweet(ctx: Context<SendTweet>, topic: String, content: String) -> ProgramResult {
    // Access "tweet" account by reference with the ability to mutate
    let tweet: &mut Account<Tweet> = &mut ctx.accounts.tweet;

    // Access author to save on "tweet" account. No need for mut because line 29
    let author: &Signer = &ctx.accounts.author;

    // Get clock for current timestamp Use unwrap to use value inside of Ok or Err
    let clock: Clock = Clock::get().unwrap();

    // We first need to access the error type like a constant
    // (ie. ErrorCode::TopicTooLong) and wrap it inside an Err enum type.
    // The into() method is a Rust feature that converts our ErrorCode into
    // whatever type is required by the code which here is Err and more
    // precisely ProgramError.
    if topic.chars().count() > 50 {
      // Return error defined in ErrorCode
      return Err(ErrorCode::TopicTooLong.into());
    }

    if content.chars().count() > 280 {
      // Return error defined in ErrorCode
      return Err(ErrorCode::ContentTooLong.into());
    }

    // Access author's key. Dereference it since it contains a reference
    tweet.author = *author.key;
    tweet.timestamp = clock.unix_timestamp;
    tweet.topic = topic;
    tweet.content = content;

    Ok(())
  }
}

#[derive(Accounts)] // Tells Anchor to generate code and macros. Boilerplate.
// <'info> is a rust  "lifetime." Tells compiler how long a variable will stay alive
pub struct SendTweet<'info> {
  // Constraint: We need to tell who is paying for the rent-exempt money
  // and how much storage we need
  // Account wraps AccountInfo in another struct that parses data accordingly.
  // This is an account of type Tweet and the data should be parsed accordingly
  #[account(init, payer = author, space = Tweet::LEN)]
  pub tweet: Account<'info, Tweet>,

  // Constraint: Author as mutable. We will mutate amount of money in their account
  // Same as AccountInfo type, except this account should sign the instruction
  #[account(mut)]
  pub author: Signer<'info>,

  // Constrait: Ensure System Program is the official one from Solana.
  // Security against malice. Require public key of account to exactly match a
  // provided public key.
  // AccountInfo can represent any account. Data will be unparsed array of bytes
  #[account(address = system_program::ID)]
  pub system_program: AccountInfo<'info> 
}

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

#[error]
pub enum ErrorCode {
  #[msg("The provided topic should be 50 characters long maximum.")]
  TopicTooLong,
  #[msg("The provided content should be 280 characters long maximum.")]
  ContentTooLong,
}
