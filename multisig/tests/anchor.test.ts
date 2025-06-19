import * as anchor from "@coral-xyz/anchor";
import { BN } from "bn.js";
import { SystemProgram, Keypair, PublicKey, Connection, clusterApiUrl, Transaction, LAMPORTS_PER_SOL } from "@solana/web3.js";
import "dotenv/config";
import { SolanaCoreMultisig } from "../target/types/solana_core_multisig";

describe("test for my multisig contract", () => {
 anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.SolanaCoreMultisig as anchor.Program<SolanaCoreMultisig>;
  const connection  =  new Connection(clusterApiUrl("devnet"));
  const multisigPDA = Keypair.generate();
  const [multisigSigner, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("multisig"), multisigPDA.publicKey.toBuffer()],
    program.programId
  );
 
  console.log("multisig PDA:", multisigPDA.publicKey.toString()); 
  console.log("multisig Signer:", multisigSigner.toString());
  console.log("bump:", bump);

  const txAccount = Keypair.generate();
  const multisigSize = 250;
  const txsize = 1000;

  const ownerA = Keypair.generate();
  const ownerB = Keypair.generate();
  const ownerC = Keypair.generate();
  const ownerD = Keypair.generate();
  const ownerE = Keypair.generate();


  it("airdrop", async () => {
  const sig  = await connection.requestAirdrop(multisigSigner, LAMPORTS_PER_SOL);
  await connection.confirmTransaction(sig, "confirmed");
  console.log("✅ Airdropped SOL to multisigSigner:", multisigSigner.toBase58());
    const sig  = await connection.requestAirdrop(txAccount.publickey, LAMPORTS_PER_SOL);
  })


  it("init multisig", async () => {
    const ownersArr = [ownerA.publicKey, ownerB.publicKey, ownerC.publicKey, ownerD.publicKey];
    const threshold = new BN(2);

    const result = await program.methods
      .initializeMultisig(ownersArr, threshold, bump)
      .accounts({
        multisig: multisigPDA.publicKey,
      })
      .preInstructions([
        await program.account.multisig.createInstruction(multisigPDA, multisigSize),
      ])
      .signers([multisigPDA])
      .rpc();

    console.log("✅ Multisig initialized:", result);
    const multisigPDAAccount = await program.account.multisig.fetch(multisigPDA.publicKey);
    console.log("Multisig Account:", multisigPDAAccount);
  });


  it("create tx account", async () => {
    const transferIx = SystemProgram.transfer({
      fromPubkey: multisigSigner,
      toPubkey: ownerA.publicKey,
      lamports: 1_000_000,
    });

    const res = await program.methods
      .createTx(transferIx.programId, transferIx.data, transferIx.keys)
      .accounts({
        multisig: multisigPDA.publicKey,
        transaction: txAccount.publicKey,
        proposer: ownerA.publicKey,
      })
      .preInstructions([
        await program.account.transaction.createInstruction(txAccount, txsize)
      ])
      .signers([ownerA, txAccount])
      .rpc();

    console.log("✅ Transaction created:", res);
  });


  it("Tx should edit successfully", async () => {
    const transferIx = SystemProgram.transfer({
      fromPubkey: multisigSigner,
      toPubkey: ownerB.publicKey,
      lamports: 500_000,
    });

    await program.methods
      .editTx(transferIx.data, transferIx.keys)
      .accounts({
        multisig: multisigPDA.publicKey,
        transaction: txAccount.publicKey,
        proposer: ownerA.publicKey,
      })
      .signers([ownerA])
      .rpc();

    console.log("✅ Tx edited by proposer");
  });


  it("tx should not be edited by non-proposer", async () => {
    const transferIx = SystemProgram.transfer({
      fromPubkey: multisigSigner,
      toPubkey: ownerC.publicKey,
      lamports: 200_000,
    });

    try {
      await program.methods
        .editTx(transferIx.data, transferIx.keys)
        .accounts({
          multisig: multisigPDA.publicKey,
          transaction: txAccount.publicKey,
          proposer: ownerC.publicKey,
        })
        .signers([ownerC])
        .rpc();
    } catch (error) {
      console.error("❌ Error editing transaction by non-proposer:", error);
    }
  });


  it("Approve the tx by ownerB", async () => {
    const res = await program.methods
      .approve()
      .accounts({
        multisig: multisigPDA.publicKey,
        transaction: txAccount.publicKey,
        proposer: ownerB.publicKey,
      })
      .signers([ownerB])
      .rpc();

    console.log("✅ Transaction approved by OwnerB:", res);
  });


  it("Approve the tx by ownerC", async () => {
    const res = await program.methods
      .approve()
      .accounts({
        multisig: multisigPDA.publicKey,
        transaction: txAccount.publicKey,
        proposer: ownerC.publicKey,
      })
      .signers([ownerC])
      .rpc();

    console.log("✅ Transaction approved by OwnerC:", res);
  });


  it("tx should not be approved by non-owner", async () => {
    try {
      await program.methods
        .approve()
        .accounts({
          multisig: multisigPDA.publicKey,
          transaction: txAccount.publicKey,
          proposer: ownerE.publicKey,
        })
        .signers([ownerE])
        .rpc();
    } catch (error) {
      console.error("❌ Error approving transaction by non-owner:", error);
    }
  });


  it("executes the transaction (manually)", async () => {
    const txOnChain = await program.account.transaction.fetch(txAccount.publicKey);

    console.log("⏳ Transaction before execution:");
    console.log("Program ID:", txOnChain.programId.toString());
    console.log("Data:", txOnChain.data.toString("hex"));
    console.log("Accounts:");
    txOnChain.accounts.forEach((acc, idx) => {
      console.log(`  [${idx}] ${acc.pubkey.toBase58()} | Signer: ${acc.isSigner} | Writable: ${acc.isWritable}`);
    });
    console.log("Signers Bitmap:", txOnChain.signers);

    const remainingAccounts = [
  ...txOnChain.accounts.map((k) => ({
    pubkey: k.pubkey,
    isSigner: k.pubkey.toBase58() === multisigSigner.toBase58() ? false : k.isSigner,
    isWritable: k.isWritable,
  })),
  {
    pubkey: txOnChain.programId,
    isSigner: false,
    isWritable: false,
  },
];
    const res = await program.methods.executeTx()
      .accounts({
        multisig: multisigPDA.publicKey,
        multisigSigner,
        transaction: txAccount.publicKey,
      })
      .remainingAccounts(remainingAccounts)
      .signers([multisigPDA])
      .rpc();

    console.log("✅ Execution result:", res);

    const txAfter = await program.account.transaction.fetch(txAccount.publicKey);
    console.log("✅ did_execute status after execution:", txAfter.didExecute);
  });


  it("cancellation of the tx account", async () => {
    await program.methods
    .cancelTx()
    .accounts({
      multisig: multisigPDA.publicKey,
      transaction: txAccount.publicKey,
      proposer: ownerA.publicKey,
  })
  .signers([ownerA])
  .rpc();
  })
});
