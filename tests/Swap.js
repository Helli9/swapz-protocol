import * as anchor from "@coral-xyz/anchor";
import {
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import { assert } from "chai";

describe("direct_token_swap (simplified)", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.DirectTokenSwap;
  const swapper = provider.wallet;

  const USDT_MINT = new anchor.web3.PublicKey("USDT_MINT_ADDRESS_HERE");
  const NU_MINT = new anchor.web3.PublicKey("NU_MINT_ADDRESS_HERE");

  let poolAuthority: anchor.web3.PublicKey;
  let usdtFrom: anchor.web3.PublicKey;
  let usdtPool: anchor.web3.PublicKey;
  let nuPool: anchor.web3.PublicKey;
  let nuTo: anchor.web3.PublicKey;

  it("Swaps USDT for NU using a shared pool PDA", async () => {
    [poolAuthority] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("authority")],
      program.programId
    );

    // Swapper's token accounts
    usdtFrom = getAssociatedTokenAddressSync(USDT_MINT, swapper.publicKey);
    nuTo = getAssociatedTokenAddressSync(NU_MINT, swapper.publicKey);

    // Pool's token accounts
    usdtPool = getAssociatedTokenAddressSync(USDT_MINT, poolAuthority, true);
    nuPool = getAssociatedTokenAddressSync(NU_MINT, poolAuthority, true);

    const amount = new anchor.BN(500_000); // 0.5 USDT (if 6 decimals)

    await program.methods
      .buyNu(amount)
      .accounts({
        swapper: swapper.publicKey,
        usdtFrom,
        usdtPool,
        nuPool,
        nuTo,
        poolAuthority,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    assert.ok(true);
  });
});
