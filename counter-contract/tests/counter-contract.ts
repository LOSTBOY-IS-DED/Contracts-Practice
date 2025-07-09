import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CounterContract } from "../target/types/counter_contract";
import { assert } from "chai";

describe("counter-contract", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const signer = provider.wallet;
  const program = anchor.workspace.counter_contract as Program<CounterContract>;

  let counterPda: anchor.web3.PublicKey;

  it("Initializes the counter", async () => {
    // Derive PDA
    [counterPda] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counter"), provider.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .initialize()
      .accounts({
        counter: counterPda,
        user: provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([signer.payer])
      .rpc();

    const counterAccount = await program.account.counter.fetch(counterPda);
    console.log("Initialized count:", counterAccount.count.toString());

    assert.ok(counterAccount.count.toNumber() === 0);
    assert.ok(counterAccount.authority.equals(provider.publicKey));
  });

  it("Increments the counter", async () => {
    await program.methods
      .increment()
      .accounts({
        counter: counterPda,
        authority: provider.publicKey,
      })
      .signers([signer.payer])
      .rpc();

    const counterAccount = await program.account.counter.fetch(counterPda);
    console.log("After increment:", counterAccount.count.toString());
    assert.equal(counterAccount.count.toNumber(), 1);
  });

  it("Decrements the counter", async () => {
    await program.methods
      .decrement()
      .accounts({
        counter: counterPda,
        authority: provider.publicKey,
      })
      .signers([signer.payer])
      .rpc();

    const counterAccount = await program.account.counter.fetch(counterPda);
    console.log("After decrement:", counterAccount.count.toString());
    assert.equal(counterAccount.count.toNumber(), 0);
  });
});
