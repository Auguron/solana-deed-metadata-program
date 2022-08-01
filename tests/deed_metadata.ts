import * as anchor from "@project-serum/anchor";
import { createHash } from "crypto";
import { Program } from "@project-serum/anchor";
import { DeedMetadata } from "../target/types/deed_metadata";

describe("deed_metadata", async () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.DeedMetadata as Program<DeedMetadata>;
  const signer = anchor.getProvider().wallet.publicKey;
  const utf8Encode = new TextEncoder();
  const property = "property";
  const buffer = createHash('sha256').update(property, 'utf8').digest();
  const [deedMetadata, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [   
          signer.toBuffer(),
          buffer,
          Buffer.from("deed_metadata"),
      ],  
      program.programId,
  );  
  const otherSigner = anchor.web3.Keypair.generate();

  it("New Deed Metadata", async () => {
    try {
        const tx = await program.methods.newDeedMetadata(
            new anchor.BN(1),
            new anchor.BN(2),
            utf8Encode.encode("usd"),
            "buyer",
            "seller",
            property,
            buffer,
            "uri",
        )
        .accounts({
            deedMetadata: deedMetadata,
            authority: signer,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
        console.log(tx);
    } catch (e) {
        console.log(e.logs);
        throw e;
    }
  });

  it("Update URI", async () => {
    try {
        const tx = await program.methods.updateUri(
            "xyz"
        )
        .accounts({
            deedMetadata: deedMetadata,
            authority: signer,
        })
        .rpc();
        console.log(tx);
    } catch (e) {
        console.log(e.logs);
        throw e;
    }
  });

  it("Set and Accept Pending Authority", async () => {
    try {
        const tx = await program.methods.setPendingAuthority(
            otherSigner.publicKey,
        )
        .accounts({
            deedMetadata: deedMetadata,
            authority: signer,
        })
        .rpc();
        console.log(tx);
    } catch (e) {
        console.log(e.logs);
        throw e;
    }
    try {
        const tx = await program.methods.acceptPendingAuthority()
        .accounts({
            deedMetadata: deedMetadata,
            pendingAuthority: otherSigner.publicKey,
        })
        .signers([otherSigner])
        .rpc();
        console.log(tx);
    } catch (e) {
        console.log(e.logs);
        throw e;
    }
    // Confirm that new authority can do something requiring authority signature
    try {
        const tx = await program.methods.updateUri(
            "qwe"
        )
        .accounts({
            deedMetadata: deedMetadata,
            authority: otherSigner.publicKey,
        })
        .signers([otherSigner])
        .rpc();
        console.log(tx);
    } catch (e) {
        console.log(e.logs);
        throw e;
    }
  });

  it("Close Deed Metadata", async () => {
    try {
        const tx = await program.methods.closeDeedMetadata()
        .accounts({
            deedMetadata: deedMetadata,
            authority: otherSigner.publicKey,
        })
        .signers([otherSigner])
        .rpc();
        console.log(tx);
    } catch (e) {
        console.log(e.logs);
        throw e;
    }
  });
});
