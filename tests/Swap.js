const anchor = require("@coral-xyz/anchor");
const BN = require("bn.js");
const assert = require("assert");
const web3 = require("@solana/web3.js");

// Mocha test suite
describe("Test", () => {
  // Set the Anchor provider (assumes env vars or Anchor.toml is set)
  anchor.setProvider(anchor.AnchorProvider.env());

  // Load the deployed program
  const program = anchor.workspace.TokenSwap;

  it("initialize", async () => {
    // Create a new Keypair for the account
    const newAccountKp = new web3.Keypair();

    // Sample data to store on-chain
    const data = new BN(42);

    // Call the initialize function from your Anchor program
    const txHash = await program.methods
      .initialize(data)
      .accounts({
        newAccount: newAccountKp.publicKey,
        signer: program.provider.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([newAccountKp])
      .rpc();

    console.log(`Use 'solana confirm -v ${txHash}' to see the logs`);

    // Wait for transaction confirmation
    await program.provider.connection.confirmTransaction(txHash);

    // Fetch the account's on-chain data
    const newAccount = await program.account.newAccount.fetch(
      newAccountKp.publicKey
    );

    console.log("On-chain data is:", newAccount.data.toString());

    // Assert that stored data matches the expected data
    assert(data.eq(newAccount.data));
  });
});
