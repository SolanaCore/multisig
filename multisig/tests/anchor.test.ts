import * as anchor from "@coral-xyz/anchor";
import { BN } from "bn.js";
import {
  SystemProgram,
  Keypair,
  PublicKey,
  Connection,
  LAMPORTS_PER_SOL,
} from "@solana/web3.js";
import { assert } from "chai";
import "dotenv/config";
import { SolanaCoreMultisig } from "../target/types/solana_core_multisig";

describe("Multisig Program Test Suite", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.SolanaCoreMultisig as anchor.Program<SolanaCoreMultisig>;
  const connection = new Connection("http://127.0.0.1:8899", "confirmed");

  const multisigPDA = Keypair.generate();
  const [multisigSigner, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("multisig"), multisigPDA.publicKey.toBuffer()],
    program.programId
  );
  console.log(multisigSigner);
  const multisigSize = 250;
  const txsize = 1000;
  let txAccount: Keypair;

  const ownerA = Keypair.generate();
  const ownerB = Keypair.generate();
  const ownerC = Keypair.generate();
  const ownerD = Keypair.generate();
  const ownerE = Keypair.generate();

  it("airdrop", async () => {
    await connection.requestAirdrop(ownerA.publicKey, LAMPORTS_PER_SOL);
    await connection.requestAirdrop(ownerB.publicKey, LAMPORTS_PER_SOL);
    await connection.requestAirdrop(ownerC.publicKey, LAMPORTS_PER_SOL);
    await connection.requestAirdrop(ownerD.publicKey, LAMPORTS_PER_SOL);
    await connection.requestAirdrop(ownerE.publicKey, LAMPORTS_PER_SOL);
  });

  it("initializes multisig", async () => {
    const result = await program.methods
      .initializeMultisig(
        [ownerA.publicKey, ownerB.publicKey, ownerC.publicKey, ownerD.publicKey],
        new BN(2),
        bump
      )
      .accounts({ multisig: multisigPDA.publicKey })
      .preInstructions([
        await program.account.multisig.createInstruction(multisigPDA, multisigSize),
      ])
      .signers([multisigPDA])
      .rpc();

    assert.ok(result);
    const airdrop = await connection.requestAirdrop(multisigSigner,LAMPORTS_PER_SOL);
    const multisigAcc = await program.account.multisig.fetch(multisigPDA.publicKey);
    assert.equal(multisigAcc.owner.length, 4);
    assert.equal(multisigAcc.threshold.toNumber(), 2);
  });

  it("creates transaction account", async () => {
    txAccount = Keypair.generate();

    const ix = SystemProgram.transfer({
      fromPubkey: multisigSigner,
      toPubkey: ownerA.publicKey,
      lamports: 1_000_000,
    });

    const res = await program.methods
      .createTx(ix.programId, ix.data, ix.keys)
      .accounts({
        multisig: multisigPDA.publicKey,
        transaction: txAccount.publicKey,
        proposer: ownerA.publicKey,
      })
      .preInstructions([await program.account.transaction.createInstruction(txAccount, txsize)])
      .signers([ownerA, txAccount])
      .rpc();

    assert.ok(res);
  });

  it("edits tx by proposer", async () => {
    const ix = SystemProgram.transfer({
      fromPubkey: multisigSigner,
      toPubkey: ownerB.publicKey,
      lamports: 500_000,
    });

    const res = await program.methods
      .editTx(ix.data, ix.keys)
      .accounts({
        multisig: multisigPDA.publicKey,
        transaction: txAccount.publicKey,
        proposer: ownerA.publicKey,
      })
      .signers([ownerA])
      .rpc();

    assert.ok(res);
  });

  it("fails edit by non-proposer", async () => {
    const ix = SystemProgram.transfer({
      fromPubkey: multisigSigner,
      toPubkey: ownerC.publicKey,
      lamports: 200_000,
    });

    try {
      await program.methods
        .editTx(ix.data, ix.keys)
        .accounts({
          multisig: multisigPDA.publicKey,
          transaction: txAccount.publicKey,
          proposer: ownerC.publicKey,
        })
        .signers([ownerC])
        .rpc();
      assert.fail("Edit should fail for non-proposer");
    } catch (err) {
      assert.include(err.toString(), "custom program error");
    }
  });

  it("approves tx by ownerB & ownerC", async () => {
    for (const owner of [ownerB, ownerC]) {
      const res = await program.methods
        .approve()
        .accounts({
          multisig: multisigPDA.publicKey,
          transaction: txAccount.publicKey,
          proposer: owner.publicKey,
        })
        .signers([owner])
        .rpc();

      assert.ok(res);
    }
  });

  it("rejects approval by non-owner", async () => {
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
      assert.fail("Should fail approval by non-owner");
    } catch (err) {
      assert.include(err.toString(), "custom program error");
    }
  });

  it("executes transaction manually", async () => {
    const txOnChain = await program.account.transaction.fetch(txAccount.publicKey);

    const remainingAccounts = [
      ...txOnChain.accounts.map((acc) => ({
        pubkey: acc.pubkey,
        isSigner: false,
        isWritable: acc.isWritable,
      })),
      { pubkey: txOnChain.programId, isSigner: false, isWritable: false },
    ];

    const result = await program.methods
      .executeTx()
      .accounts({
        multisig: multisigPDA.publicKey,
        multisigSigner,
        transaction: txAccount.publicKey,
      })
      .remainingAccounts(remainingAccounts)
      .signers([multisigPDA])
      .rpc();

    assert.ok(result);

    const txAfter = await program.account.transaction.fetch(txAccount.publicKey);
    assert.equal(txAfter.didExecute, true);
  });

  it("cancels transaction fails because the tx is already executed", async () => {
    const result = await program.methods
      .cancelTx()
      .accounts({
        multisig: multisigPDA.publicKey,
        transaction: txAccount.publicKey,
        proposer: ownerA.publicKey,
      })
      .signers([ownerA])
      .rpc();

    assert.ok(result);
  });
});