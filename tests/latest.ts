import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Latest } from "../target/types/latest";
import { assert } from "chai";

describe("latest", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.latest as Program<Latest>;

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("Transfers SOL using send_sol", async () => {
    const provider = anchor.getProvider();
    const sender = provider.wallet.publicKey;
    const receiver = provider.wallet.publicKey; // send to self for test
    const amount = new anchor.BN(1_000_000); // 0.001 SOL

    // Get initial balance
    const before = await provider.connection.getBalance(sender);

    // Invoke send_sol
    const txSig = await program.methods
      .sendSol(amount)
      .accounts({
        sender,
        receiver,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    console.log("send_sol tx", txSig);

    // Get post‑balance and assert decrease by amount (plus fee)
    const after = await provider.connection.getBalance(sender);
    assert.isAtMost(after, before - amount.toNumber(), "Balance should decrease by at least the sent amount");
  });
});
