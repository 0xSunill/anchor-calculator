import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorProgram } from "../target/types/anchor_program";
import { assert } from "chai";
// import { AnchorProgram } from "../target/types/anchor_program";

describe("anchor-program", () => {


  anchor.setProvider(anchor.AnchorProvider.env());

  const newAccount = anchor.web3.Keypair.generate();

  const program = anchor.workspace.anchorProgram as Program<AnchorProgram>;

  it("Is initialized!", async () => {

    const tx = await program.methods.initialize(7)
      .accounts({
        signer: anchor.getProvider().wallet.publicKey,
        account: newAccount.publicKey
      })
      .signers([newAccount])
      .rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.dataShape.fetch(newAccount.publicKey)
    assert(account.numb == 7)
  });


  it("Is double!", async () => {

    const tx = await program.methods.double()
      .accounts({
        signer: anchor.getProvider().wallet.publicKey,
        account: newAccount.publicKey
      })

      .rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.dataShape.fetch(newAccount.publicKey)
    assert(account.numb == 14)
  });


  it("Is add!", async () => {

    const tx = await program.methods.add(6)
      .accounts({
        signer: anchor.getProvider().wallet.publicKey,
        account: newAccount.publicKey
      })

      .rpc();
    console.log("Your transaction signature", tx);
    const account = await program.account.dataShape.fetch(newAccount.publicKey)
    assert(account.numb == 20)
  });
});
