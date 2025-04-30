import * as anchor from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { assert } from "chai";

describe("direct_token_swap (simplified)", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.DirectTokenSwap;
  const swapper = provider.wallet;

  // Define mint addresses (replace with actual mint addresses)
  const USDT_MINT = new anchor.web3.PublicKey("USDT_MINT_ADDRESS_HERE");
  const NU_MINT = new anchor.web3.PublicKey("NU_MINT_ADDRESS_HERE");

  let poolAuthority: anchor.web3.PublicKey;
  let usdt: anchor.web3.PublicKey;
  let pool: anchor.web3.PublicKey;
  let nu: anchor.web3.PublicKey;

  it("Swaps USDT for NU using a shared pool PDA", async () => {
    [poolAuthority] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("authority")],
      program.programId
    );

    // Get associated token accounts (must exist)
    usdt = getAssociatedTokenAddressSync(USDT_MINT, swapper.publicKey);
    nu = getAssociatedTokenAddressSync(NU_MINT, swapper.publicKey);
    pool = getAssociatedTokenAddressSync(NU_MINT, poolAuthority, true); // Assuming pool holds NU tokens

    const amount = new anchor.BN(500_000); // 0.5 USDT (assuming 6 decimals)

    await program.methods
      .buyNu(amount)
      .accounts({
        swapper: swapper.publicKey,
        usdt,
        pool,
        nu,
        poolAuthority,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    assert.ok(true); 
  });
});