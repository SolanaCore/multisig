import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaCoreMultisig } from "../target/types/multisig";
import { SystemProgram, Keypair, PublicKey } from "@solana/web3.js";

// -------------------- Setup --------------------
// Configure the client to use the cluster.

anchor.setProvider(anchor.getProvider());
const program = anchor.workspace.SolanaCoreMultisig as Program<SolanaCoreMultisig>;
// Multisig Keypairs
const multisigSigner = Keypair.generate();
const [multisigPDA, bump] = PublicKey.findProgramAddressSync(
  [multisigSigner.publicKey.toBuffer()],
  program.programId
);
console.log(multisigSigner.publicKey.toString());

const txAccount = Keypair.generate();
const multisigSize = 250;
const txsize = 1000;

// Owners
const ownerA = Keypair.generate();
const ownerB = Keypair.generate();
const ownerC = Keypair.generate();
const ownerD = Keypair.generate();
const ownerE = Keypair.generate();

describe("test for my multisig contract", () => {
  it("init multisig", async () => {
    const ownersArr = [
      ownerA.publicKey,
      ownerB.publicKey,
      ownerC.publicKey,
      ownerD.publicKey,
    ];
    const threshold = new anchor.BN(3);

    await program.methods
      .initializeMultisig(ownersArr, threshold, bump)
      .accounts({
        multisig: multisigSigner.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .preInstructions([
        await program.account.multisig.createInstruction(
          multisigSigner,
          multisigSize
        ),
      ])
      .signers([multisigSigner])
      .rpc();
  });

  it("create tx", async () => {
    const newOwners = [
      ownerA.publicKey,
      ownerB.publicKey,
      ownerC.publicKey,
      ownerD.publicKey,
      ownerE.publicKey,
    ];
    const ixDATA = await program.coder.instruction.encode(
      "changeOwners",
      newOwners
    );
    const ix = await program.methods
      .changeOwners(newOwners)
      .accounts({
        multisig: multisigSigner.publicKey,
        multisigSigner: multisigPDA,
      })
      .instruction();
    console.log("-------------------------------------");
    console.log(ix.data);
    console.log("--------------------------------------");
    // Use the correct program ID, accounts and data
    await program.methods
      .createTx(ix.programId, ix.data, ix.keys)
      .accounts({
        multisig: multisigSigner.publicKey,
        transaction: txAccount.publicKey,
        proposer: ownerA.publicKey,
      })
      .preInstructions([
        await program.account.transaction.createInstruction(txAccount, txsize),
      ])
      .signers([ownerA, txAccount])
      .rpc();
  });

  it("approve tx", async () => {
    const signers = [ownerB, ownerC, ownerD];
    for (const s of signers) {
      await program.methods
        .approve()
        .accounts({
          signer: s.publicKey,
          multisig: multisigSigner.publicKey,
          transaction: txAccount.publicKey,
        })
        .signers([s])
        .rpc();
    }
  });

  it("exec tx", async () => {
    const newOwners = [
      ownerA.publicKey,
      ownerB.publicKey,
      ownerC.publicKey,
      ownerD.publicKey,
      ownerE.publicKey,
    ];

    // Create the same instruction again
    const ix = await program.methods
      .changeOwners(newOwners)
      .accounts({
        multisig: multisigSigner.publicKey,
        multisigSigner: multisigPDA,
      })
      .instruction();

    // Properly map keys to `remainingAccounts`
    const remaining = ix.keys.map((k) => ({
      pubkey: k.pubkey,
      isSigner: !k.pubkey.equals(multisigPDA), // Only PDA signs
      isWritable: k.isWritable,
    }));

    remaining.push({
      pubkey: program.programId,
      isSigner: false,
      isWritable: false,
    });
    try {
      const res = await program.methods
        .executeTx()
        .accounts({
          multisig: multisigSigner.publicKey,
          multisigSigner: multisigPDA,
          tx: txAccount.publicKey,
        })
        .remainingAccounts(remaining)
        .signers([multisigSigner])
        .rpc(); // Do not sign with PDA

      console.log(res);
    } catch (error) {
      console.log(error);
    }

    let multisigAccount = await program.account.multisig.fetch(
      multisigSigner.publicKey
    );
    console.log(
      multisigAccount.owner.length,
      multisigAccount.owner.forEach((s) => {
        console.log(s.toString());
      })
    );
    // assert.strictEqual(multisigAccount.owner.length, 5);

    let txAcc = await program.account.transaction.fetch(txAccount.publicKey);
    console.log(txAcc);
  });
});

