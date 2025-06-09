import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorCal } from "../target/types/anchor_cal";
import { it } from "mocha";
import { assert } from "chai";

describe("anchor-calculator", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.anchorCal as Program<AnchorCal>;
  const dataAccount = anchor.web3.Keypair.generate();

  it("Is initialised", async () => {
    const tx = await program.methods
      .init(10)
      .accounts({
        account: dataAccount.publicKey,
        signer: anchor.getProvider().wallet.publicKey,
      })
      .signers([dataAccount])
      .rpc();
    console.log("Transaction signature", tx);
  });

  it("Is addition ", async () => {
    const tx = await program.methods
      .add(10)
      .accounts({
        account: dataAccount.publicKey,
        signer: anchor.getProvider().wallet.publicKey,
      })
      .rpc();

    console.log("Transaction signature", tx);
    const account = await program.account.dataShape.fetch(dataAccount.publicKey);
    assert.equal(account.num, 20);
  });

  it("Is subtraction ", async () => {
    const tx = await program.methods
      .sub(10)
      .accounts({
        account: dataAccount.publicKey,
        signer: anchor.getProvider().wallet.publicKey,
      })
      .rpc();

    console.log("Transaction signature", tx);
    const account = await program.account.dataShape.fetch(
      dataAccount.publicKey
    );
    assert.equal(account.num, 10);
  });
   
  it("Is multiply ", async () => {
    const tx = await program.methods
      .mul(2)
      .accounts({
        account: dataAccount.publicKey,
        signer: anchor.getProvider().wallet.publicKey,
      })
      .rpc();

    console.log("Transaction signature", tx);
    const account = await program.account.dataShape.fetch(
      dataAccount.publicKey
    );
    assert.equal(account.num, 20);
  });

  it("Is division ", async () => {
    const tx = await program.methods
      .div(5)
      .accounts({
        account: dataAccount.publicKey,
        signer: anchor.getProvider().wallet.publicKey,
      })
      .rpc();

    console.log("Transaction signature", tx);
    const account = await program.account.dataShape.fetch(
      dataAccount.publicKey
    );
    assert.equal(account.num, 4);
  });

  it("Is double ", async () => {
    const tx = await program.methods
      .double()
      .accounts({
        account: dataAccount.publicKey,
        signer: anchor.getProvider().wallet.publicKey,
      })
      .rpc();

    console.log("Transaction signature", tx);
    const account = await program.account.dataShape.fetch(
      dataAccount.publicKey
    );
    assert.equal(account.num, 8);
  });

  it("Is half ", async () => {
    const tx = await program.methods
      .half()
      .accounts({
        account: dataAccount.publicKey,
        signer: anchor.getProvider().wallet.publicKey,
      })
      .rpc();

    console.log("Transaction signature", tx);
    const account = await program.account.dataShape.fetch(
      dataAccount.publicKey
    );
    assert.equal(account.num, 4);
  });

});
