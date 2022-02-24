import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { SolanaTwitter } from '../target/types/solana_twitter';
import * as assert from "assert";
import * as bs58 from "bs58";

describe('solana-twitter', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SolanaTwitter as Program<SolanaTwitter>;

  it('can send a new tweet', async () => {
    const tweet = anchor.web3.Keypair.generate(); // Generate a keypair for accounts (publicKey) and signers

    await program.rpc.sendTweet("veganism", "Hummus is good", {
      accounts: {
        tweet: tweet.publicKey,
        author: program.provider.wallet.publicKey, // Access our wallet's public key
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [tweet]
    });

    // Fetch the account details of the created tweet
    const tweetAccount = await program.account.tweet.fetch(tweet.publicKey)

    // Author and wallet have different references, so we convert to base 58 so they'll only be equal if the strings are the same
    assert.equal(tweetAccount.author.toBase58(), program.provider.wallet.publicKey.toBase58())
    assert.equal(tweetAccount.topic, "veganism");
    assert.equal(tweetAccount.content, "Hummus is good");
    assert.ok(tweetAccount.timestamp);
  });

  it('can send a new tweet without a topic', async () => {
    const tweet = anchor.web3.Keypair.generate(); // Generate a keypair for accounts (publicKey) and signers

    await program.rpc.sendTweet("", "Good morning", {
      accounts: {
        tweet: tweet.publicKey,
        author: program.provider.wallet.publicKey, // Access our wallet's public key
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [tweet]
    });

    const tweetAccount = await program.account.tweet.fetch(tweet.publicKey)

    // Author and wallet have different references, so we convert to base 58 so they'll only be equal if the strings are the same
    assert.equal(tweetAccount.author.toBase58(), program.provider.wallet.publicKey.toBase58())
    assert.equal(tweetAccount.topic, "");
    assert.equal(tweetAccount.content, "Good morning");
    assert.ok(tweetAccount.timestamp);
  });

  it('can send a new tweet from a different author', async () => {
    const otherUser = anchor.web3.Keypair.generate();

    // Generate another user and airdrop them some SOL
    const signature = await program.provider.connection.requestAirdrop(otherUser.publicKey, 1000000000);
    await program.provider.connection.confirmTransaction(signature);

    const tweet = anchor.web3.Keypair.generate(); // Generate a keypair for accounts (publicKey) and signers

    // Call sendTweet on behalf of this other user
    await program.rpc.sendTweet("veganism", "Tofu is good", {
      accounts: {
        tweet: tweet.publicKey,
        author: otherUser.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers: [otherUser, tweet]
    });

    const tweetAccount = await program.account.tweet.fetch(tweet.publicKey)

    // Author and wallet have different references, so we convert to base 58 so they'll only be equal if the strings are the same
    assert.equal(tweetAccount.author.toBase58(), otherUser.publicKey.toBase58())
    assert.equal(tweetAccount.topic, "veganism");
    assert.equal(tweetAccount.content, "Tofu is good");
    assert.ok(tweetAccount.timestamp);
  });

  it('cannot provide a topic with over 50 characters', async () => {
    try {
      const tweet = anchor.web3.Keypair.generate(); // Generate a keypair for accounts (publicKey) and signers
      const topicWith51Chars = "x".repeat(51);
      await program.rpc.sendTweet(topicWith51Chars, "Hummus is good", {
        accounts: {
          tweet: tweet.publicKey,
          author: program.provider.wallet.publicKey, // Access our wallet's public key
          systemProgram: anchor.web3.SystemProgram.programId
        },
        signers: [tweet]
      });
    }
    catch(error) {
      assert.equal(error.msg, "The provided topic should be 50 characters long maximum.");
      return;
    }

    assert.fail("The instruction should have failed with a 51-character topic.");
  });

  it('cannot provide content with over 280 characters', async () => {
    try {
      const tweet = anchor.web3.Keypair.generate(); // Generate a keypair for accounts (publicKey) and signers
      const contentWith281Chars = "x".repeat(281);
      await program.rpc.sendTweet("veganism", contentWith281Chars, {
        accounts: {
          tweet: tweet.publicKey,
          author: program.provider.wallet.publicKey, // Access our wallet's public key
          systemProgram: anchor.web3.SystemProgram.programId
        },
        signers: [tweet]
      });
    }
    catch(error) {
      assert.equal(error.msg, "The provided content should be 280 characters long maximum.");
      return;
    }

    assert.fail("The instruction should have failed with a 281-character topic.");
  });

  it('can fetch all tweets', async() => {
    const tweetAccounts = await program.account.tweet.all();
    assert.equal(tweetAccounts.length, 3);
  });

  it('can filter tweets by author', async() => {
    const authorPublicKey = await program.provider.wallet.publicKey;
    const tweetAccounts = await program.account.tweet.all([
      {
        memcmp: {
          offset: 8, // Length of discriminator
          bytes: authorPublicKey.toBase58()
        }
      }
    ]);

    assert.equal(tweetAccounts.length, 2);
    assert.ok(tweetAccounts.every(tweetAccount => { // every() returns true if the provided callback returns true for every account
      return tweetAccount.account.author.toBase58() === authorPublicKey.toBase58();
    }));
  });

  it('can filter tweets by topics', async() => {
    const tweetAccounts = await program.account.tweet.all([
      {
        memcmp: {
          offset: 8 + // Discriminator
            32 + // Author's public key
            8 + // Timestamp
            4, // Topic string prefix
          bytes: bs58.encode(Buffer.from('veganism'))
        }
      }
    ])

    assert.equal(tweetAccounts.length, 2);
    assert.ok(tweetAccounts.every(tweetAccount => {
      return tweetAccount.account.topic === "veganism";
    }));
  });
});
