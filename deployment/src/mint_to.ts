import * as anchor from '@coral-xyz/anchor';
import BN from 'bn.js';
import { getAssociatedTokenAddress, createAssociatedTokenAccountInstruction } from "@solana/spl-token";
import { getAccount, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair, PublicKey, SystemProgram, Transaction } from "@solana/web3.js";
import idl from "./idl/tokenizer.json" with { type: 'json' }
import { createSignerFromKeypair, signerIdentity, none, createUmi } from "@metaplex-foundation/umi";
import { readFileSync } from "fs";
import { resolve } from "path";
import { Program, web3 } from "@coral-xyz/anchor";

//Insert keypair paths below
const keypair1 = Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(readFileSync(resolve(""), "utf-8")))
  );
  const keypair2 = Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(readFileSync(resolve(""), "utf-8")))
);
const recipient  = keypair1.publicKey;
const connection = new anchor.web3.Connection("https://api.testnet.solana.com", "confirmed");
const wallet = new anchor.Wallet(keypair1);
const provider = new anchor.AnchorProvider(connection, wallet, {
  commitment: 'confirmed',
});
anchor.setProvider(provider);

const programId = new web3.PublicKey((idl as any).metadata.address);
const program = new Program(idl as any, programId, provider);

const splMint = new PublicKey("GCBFs8zj6KuUAzAeNskF7fhqmGgd42uXD9FMcnQSGo7b");
const customMint = PublicKey.findProgramAddressSync(
  [Buffer.from("custom_mint"), splMint.toBuffer()],
  program.programId
)[0];

const mintAuthority = PublicKey.findProgramAddressSync(
  [Buffer.from("mint_auth"), splMint.toBuffer()],
  program.programId
)[0];

const mintTo = async () => {
  const ata = await getAssociatedTokenAddress(
    splMint,
    keypair1.publicKey,
    false
  );

  // Check if ATA exists
  let ataExists = true;
  try {
    await getAccount(connection, ata);
    console.log("ATA already exists:", ata.toBase58());
  } catch (e) {
    console.log("ATA does not exist, will create:", ata.toBase58());
    ataExists = false;
  }

  // If it doesn't exist, create ATA before minting
  if (!ataExists) {
    const ataIx = createAssociatedTokenAccountInstruction(
      keypair1.publicKey,
      ata,
      keypair1.publicKey,
      splMint
    );

    const tx = new Transaction().add(ataIx);
    await provider.sendAndConfirm(tx, [keypair1]);
    console.log("ATA created successfully.");
  }

  // Signers who approve the mint (multisig)
  const signers = [keypair1.publicKey, keypair2.publicKey];
  // Execute the mint instruction
  const tx = await program.methods
    .mintTo(new BN("9000000000"), signers)
    .accounts({
      payer: keypair1.publicKey,
      customMint,
      splMint,
      recipient: ata,
      mintAuthority,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: web3.SystemProgram.programId,
      rent: web3.SYSVAR_RENT_PUBKEY,
    })
    .signers([keypair1])
    .rpc();

  console.log("Minted successfully in tx:", tx);
};

mintTo().catch(console.error);
