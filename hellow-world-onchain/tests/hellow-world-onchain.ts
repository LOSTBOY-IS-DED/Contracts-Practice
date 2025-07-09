import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HellowWorldOnchain } from "../target/types/hellow_world_onchain";
import { assert } from "chai";

describe("hello-world-anchor", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .hellow_world_onchain as Program<HellowWorldOnchain>;
  const user = provider.wallet;

  let baseAccountPDA: anchor.web3.PublicKey;
  let bump: number;

  const seed = Buffer.from("greeting");

  before(async () => {
    [baseAccountPDA, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [seed, user.publicKey.toBuffer()],
      program.programId
    );
  });

  it("Initializes the greeting account", async () => {
    await program.methods
      .initialize("Hello, Solana!")
      .accounts({
        baseAccount: baseAccountPDA,
        user: user.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([]) // No need to sign since PDA is program-owned
      .rpc();

    const account = await program.account.greetingAccount.fetch(baseAccountPDA);
    assert.equal(account.greeting, "Hello, Solana!");
  });

  it("Updates the greeting", async () => {
    await program.methods
      .update("Updated Greeting")
      .accounts({
        baseAccount: baseAccountPDA,
        authority: user.publicKey,
      })
      .rpc();

    const account = await program.account.greetingAccount.fetch(baseAccountPDA);
    assert.equal(account.greeting, "Updated Greeting");
  });
});
