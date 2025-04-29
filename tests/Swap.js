const anchor = require("@coral-xyz/anchor");
const assert = require("assert");
const { Token, createMint, getOrCreateAssociatedTokenAccount, mintTo } = require("@solana/spl-token");
const { SystemProgram, Keypair, PublicKey } = require("@solana/web3.js");

describe("buy_nu test", async () => {
  // Configure the client
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.YourProgramName;

  const user = provider.wallet;
  let usdtMint, nuMint;
  let userUsdtAta, userNuAta;
  let usdtPool, nuPool;
  let poolAuthority;

  const amountToBuy = new anchor.BN(100); // 100 NU

  it("Initializes mints and ATAs", async () => {
    // 1. Create mock USDT and NU mints
    usdtMint = await createMint(
      provider.connection,
      user.payer,
      user.publicKey,
      null,
      6
    );
    nuMint = await createMint(
      provider.connection,
      user.payer,
      user.publicKey,
      null,
      6
    );

    // 2. Create ATAs
    userUsdtAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user.payer,
      usdtMint,
      user.publicKey
    );

    userNuAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user.payer,
      nuMint,
      user.publicKey
    );

    // 3. Derive PDA authority for pool
    [poolAuthority] = await PublicKey.findProgramAddressSync(
      [Buffer.from("authority")],
      program.programId
    );

    // 4. Create pool token accounts
    usdtPool = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user.payer,
      usdtMint,
      poolAuthority,
      true
    );

    nuPool = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user.payer,
      nuMint,
      poolAuthority,
      true
    );

    // 5. Fund user and NU pool
    await mintTo(
      provider.connection,
      user.payer,
      usdtMint,
      userUsdtAta.address,
      user.payer,
      1000_000_000 // 1000 USDT
    );

    await mintTo(
      provider.connection,
      user.payer,
      nuMint,
      nuPool.address,
      user.payer,
      1000_000_000 // 1000 NU
    );
  });

  it("Buys NU using USDT", async () => {
    await program.methods
      .buyNu(amountToBuy)
      .accounts({
        swapper: user.publicKey,
        usdtFrom: userUsdtAta.address,
        usdtPool: usdtPool.address,
        nuPool: nuPool.address,
        nuTo: userNuAta.address,
        poolAuthority: poolAuthority,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([])
      .rpc();

    // Check balances
    const updatedUserUsdt = await provider.connection.getTokenAccountBalance(userUsdtAta.address);
    const updatedUserNu = await provider.connection.getTokenAccountBalance(userNuAta.address);

    console.log("User USDT:", updatedUserUsdt.value.uiAmount);
    console.log("User NU:", updatedUserNu.value.uiAmount);

    assert.strictEqual(Number(updatedUserNu.value.amount), amountToBuy.toNumber());
  });
});
