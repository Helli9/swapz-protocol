const anchor = require("@coral-xyz/anchor");
const assert = require("assert");
const { Token, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } = require("@solana/spl-token");
const web3 = require("@solana/web3.js");

// Use the program ID defined in your Rust code
const programId = new anchor.web3.PublicKey("3At9UEz1bGW2ofW4twm4EBEmz6XRB22K19PubbmJGNP2"); // *** IMPORTANT: REPLACE THIS with your actual program ID ***

// In a real test environment, especially locally, relying on standard
// testnet/devnet mints like actual JUP or the native SOL mint can be tricky
// for controlled testing like minting.
// For robust testing, it's common practice to create dummy mints within the test itself.
// This allows the test to have minting authority and control token supply.
// We will create dummy mints for 'SOL' and 'JUP' for this test.
// If you specifically need to test with actual WSOL and JUP on a network,
// the setup would involve getting tokens from faucets or exchanges instead of minting.

describe("direct_token_swap", () => {
    // Configure the client to use the local cluster.
    // Make sure you have a local validator running (`solana-test-validator`).
    const provider = anchor.AnchorProvider.local();
    anchor.setProvider(provider);

    // Load the program's IDL and create a program client.
    // The IDL is generated when you build your program (`anchor build`).
    const program = new anchor.Program(require("../target/idl/direct_token_swap.json"), programId, provider);

    // Keypairs for the two swappers. These will need to sign the transaction.
    let swapperA = anchor.web3.Keypair.generate();
    let swapperB = anchor.web3.Keypair.generate();

    let tokenSolFrom; // Swapper A's ATA for dummy SOL
    let tokenSolTo;   // Swapper B's ATA for dummy SOL
    let tokenJupFrom; // Swapper B's ATA for dummy JUP
    let tokenJupTo;   // Swapper A's ATA for dummy JUP

    // Keypairs for the dummy token mints. We'll create these within the test.
    let dummySolMintKeypair = anchor.web3.Keypair.generate();
    let dummyJupMintKeypair = anchor.web3.Keypair.Keypair.generate();

    // Public Keys for the dummy token mints.
    let dummySolMint;
    let dummyJupMint;


    // The amount of tokens to swap
    const swapAmount = new anchor.BN(100); // Example amount. Adjust based on your token decimals if needed.

    before(async () => {

        console.log("Setting up test environment...");

        // 1. Airdrop SOL to swapper accounts
        // Swappers need SOL to pay for transaction fees and account rent.
        const solAirdropAmount = 100 * anchor.web3.LAMPORTS_PER_SOL; // A generous amount
        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(swapperA.publicKey, solAirdropAmount),
            "confirmed"
        );

        await provider.connection.confirmTransaction(
            await provider.connection.requestAirdrop(swapperB.publicKey, solAirdropAmount),
            "confirmed"
        );

        // 2. Create Dummy Token Mints
        // We create new mints controlled by the provider's wallet for easy token distribution.
        // The provider's wallet will be the mint authority.

        console.log("Creating dummy SOL and JUP token mints...");

        dummySolMint = await Token.createMint(
            provider.connection,
            provider.wallet.payer, // Payer for the transaction fee
            provider.wallet.publicKey, // Mint Authority: The provider's wallet can mint tokens
            null, // Freeze Authority: No freeze authority
            9, // Decimals (using 9 like native SOL)
            TOKEN_PROGRAM_ID, // The SPL Token Program ID
            dummySolMintKeypair // Provide the keypair for the new mint account
        );
        console.log(`Dummy SOL Mint created: ${dummySolMint.toBase58()}`);


        dummyJupMint = await Token.createMint(
            provider.connection,
            provider.wallet.payer, // Payer
            provider.wallet.publicKey, // Mint Authority
            null, // Freeze Authority
            6, // Decimals (using 6, common for JUP)
            TOKEN_PROGRAM_ID,
            dummyJupMintKeypair // Provide the keypair for the new mint account
        );
        console.log(`Dummy JUP Mint created: ${dummyJupMint.toBase58()}`);


        // 3. Get or Create Associated Token Accounts (ATAs)
        // These are the accounts that will hold the tokens for each swapper.
        // We use the SPL Associated Token Account program to derive and create these.

        console.log("Creating Associated Token Accounts (ATAs)...");

        // Swapper A's dummy SOL Token Account (where Swapper A holds SOL)
        tokenSolFrom = await Token.getAssociatedTokenAddress(
            dummySolMint,       // Mint Address
            swapperA.publicKey, // Owner Address
            false,              // allowOwnerOffCurve (set to false for standard wallets)
            TOKEN_PROGRAM_ID,   // SPL Token Program ID
            ASSOCIATED_TOKEN_PROGRAM_ID // SPL Associated Token Account Program ID
        );
        await Token.createAssociatedTokenAccount(
            provider.connection,
            provider.wallet.payer, // Payer for creating the ATA account
            dummySolMint,       // Mint Address
            swapperA.publicKey, // Owner Address
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );
        console.log(`Swapper A's Dummy SOL ATA: ${tokenSolFrom.toBase58()}`);


        // Swapper B's dummy SOL Token Account (where Swapper B will receive SOL)
        tokenSolTo = await Token.getAssociatedTokenAddress(
            dummySolMint,
            swapperB.publicKey,
            false,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );
        await Token.createAssociatedTokenAccount(
            provider.connection,
            provider.wallet.payer, // Payer
            dummySolMint,
            swapperB.publicKey,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );
         console.log(`Swapper B's Dummy SOL ATA: ${tokenSolTo.toBase58()}`);


        // Swapper B's dummy JUP Token Account (where Swapper B holds JUP)
        tokenJupFrom = await Token.getAssociatedTokenAddress(
            dummyJupMint,
            swapperB.publicKey,
            false,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );
         await Token.createAssociatedTokenAccount(
            provider.connection,
            provider.wallet.payer, // Payer
            dummyJupMint,
            swapperB.publicKey,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );
        console.log(`Swapper B's Dummy JUP ATA: ${tokenJupFrom.toBase58()}`);


        // Swapper A's dummy JUP Token Account (where Swapper A will receive JUP)
        tokenJupTo = await Token.getAssociatedTokenAddress(
            dummyJupMint,
            swapperA.publicKey,
            false,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );
        await Token.createAssociatedTokenAccount(
            provider.connection,
            provider.wallet.payer, // Payer
            dummyJupMint,
            swapperA.publicKey,
            TOKEN_PROGRAM_ID,
            ASSOCIATED_TOKEN_PROGRAM_ID
        );
         console.log(`Swapper A's Dummy JUP ATA: ${tokenJupTo.toBase58()}`);


        // 4. Mint Tokens into the 'from' accounts
        // Swapper A needs dummy SOL, Swapper B needs dummy JUP

        console.log("Minting tokens into 'from' ATAs...");

        // Mint dummy SOL to Swapper A's dummy SOL ATA
        const mintAmountSol = swapAmount.mul(new anchor.BN(2)); // Mint more than the swap amount
        await Token.mintTo(
            provider.connection,
            provider.wallet.payer, // Payer
            dummySolMint, // Mint account public key
            tokenSolFrom, // Destination token account public key
            provider.wallet.publicKey, // Mint Authority public key (provider's wallet)
            mintAmountSol, // Amount to mint
            [provider.wallet.payer] // Signers for the mint authority (provider's wallet signs)
        );
        console.log(`Minted ${mintAmountSol.toString()} dummy SOL to Swapper A's ATA`);

        // Mint dummy JUP to Swapper B's dummy JUP ATA
        const mintAmountJup = swapAmount.mul(new anchor.BN(2)); // Mint more than the swap amount
         await Token.mintTo(
            provider.connection,
            provider.wallet.payer, // Payer
            dummyJupMint, // Mint account public key
            tokenJupFrom, // Destination token account public key
            provider.wallet.publicKey, // Mint Authority public key (provider's wallet)
            mintAmountJup, // Amount to mint
            [provider.wallet.payer] // Signers for the mint authority
        );
        console.log(`Minted ${mintAmountJup.toString()} dummy JUP to Swapper B's ATA`);

        console.log("Test environment setup complete.");
    });

    it("Performs the token swap correctly", async () => {
        // This is the main test case for the swap instruction.

        console.log("\nStarting swap test...");

        // Get initial token balances of the accounts before the swap
        let tokenSolFromAccountBefore = await Token.getAccountInfo(provider.connection, tokenSolFrom);
        let tokenSolToAccountBefore = await Token.getAccountInfo(provider.connection, tokenSolTo);
        let tokenJupFromAccountBefore = await Token.getAccountInfo(provider.connection, tokenJupFrom);
        let tokenJupToAccountBefore = await Token.getAccountInfo(provider.connection, tokenJupTo);

        console.log("Initial Balances:");
        console.log(`Swapper A SOL: ${tokenSolFromAccountBefore.amount.toString()}`);
        console.log(`Swapper B SOL: ${tokenSolToAccountBefore.amount.toString()}`);
        console.log(`Swapper B JUP: ${tokenJupFromAccountBefore.amount.toString()}`);
        console.log(`Swapper A JUP: ${tokenJupToAccountBefore.amount.toString()}`);


        // Call the swap instruction on your program
        // This is where the actual program logic is executed.
        // We pass the required accounts and the swap amount.
        // Crucially, we include both swapperA and swapperB keypairs in the .signers() array,
        // as your program requires both to sign.
        try {
             const tx = await program.methods
                .swap(swapAmount)
                .accounts({
                    tokenSolFrom: tokenSolFrom, // Swapper A's SOL ATA (from)
                    tokenSolTo: tokenSolTo,     // Swapper B's SOL ATA (to)
                    tokenJupFrom: tokenJupFrom, // Swapper B's JUP ATA (from)
                    tokenJupTo: tokenJupTo,     // Swapper A's JUP ATA (to)
                    swapperA: swapperA.publicKey, // Swapper A's public key (signer)
                    swapperB: swapperB.publicKey, // Swapper B's public key (signer)
                    tokenProgram: TOKEN_PROGRAM_ID, // The SPL Token Program ID
                })
                .signers([swapperA, swapperB]) // *** IMPORTANT: Both swapper keypairs must sign ***
                .rpc(); // Send the transaction to the cluster

            console.log("Swap transaction successful. Signature:", tx);

        } catch (error) {
            console.error("Swap transaction failed:", error);
            // Re-throw the error to fail the test
            throw error;
        }


        // Fetch token balances after the swap
        let tokenSolFromAccountAfter = await Token.getAccountInfo(provider.connection, tokenSolFrom);
        let tokenSolToAccountAfter = await Token.getAccountInfo(provider.connection, tokenSolTo);
        let tokenJupFromAccountAfter = await Token.getAccountInfo(provider.connection, tokenJupFrom);
        let tokenJupToAccountAfter = await Token.getAccountInfo(provider.connection, tokenJupTo);

        console.log("Balances After Swap:");
        console.log(`Swapper A SOL: ${tokenSolFromAccountAfter.amount.toString()}`);
        console.log(`Swapper B SOL: ${tokenSolToAccountAfter.amount.toString()}`);
        console.log(`Swapper B JUP: ${tokenJupFromAccountAfter.amount.toString()}`);
        console.log(`Swapper A JUP: ${tokenJupToAccountAfter.amount.toString()}`);

        // 5. Assert that the balances have changed as expected
        // Swapper A sends swapAmount of dummy SOL and receives swapAmount of dummy JUP
        assert.strictEqual(
            tokenSolFromAccountAfter.amount.toString(),
            tokenSolFromAccountBefore.amount.sub(swapAmount).toString(),
            "Swapper A's dummy SOL balance should decrease by swapAmount"
        );
        assert.strictEqual(
            tokenJupToAccountAfter.amount.toString(),
            tokenJupToAccountBefore.amount.add(swapAmount).toString(),
            "Swapper A's dummy JUP balance should increase by swapAmount"
        );

        // Swapper B sends swapAmount of dummy JUP and receives swapAmount of dummy SOL
        assert.strictEqual(
            tokenJupFromAccountAfter.amount.toString(),
            tokenJupFromAccountBefore.amount.sub(swapAmount).toString(),
            "Swapper B's dummy JUP balance should decrease by swapAmount"
        );
         assert.strictEqual(
            tokenSolToAccountAfter.amount.toString(),
            tokenSolToAccountBefore.amount.add(swapAmount).toString(),
            "Swapper B's dummy SOL balance should increase by swapAmount"
        );

        console.log("Swap successful and balances verified!");

        // You could also add checks for the emitted 'SwapExecuted' event here if needed.
        // This typically involves fetching the transaction logs and parsing them, which adds complexity.
        // Checking the account balances is often sufficient for a functional test.
    });
});