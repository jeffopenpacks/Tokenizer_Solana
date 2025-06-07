import * as anchor from '@coral-xyz/anchor';
import { MPL_TOKEN_METADATA_PROGRAM_ID } from '@metaplex-foundation/mpl-token-metadata';
import { readFileSync } from 'fs';
import BN from 'bn.js';
import { Keypair, PublicKey, SystemProgram, SYSVAR_INSTRUCTIONS_PUBKEY } from '@solana/web3.js';
import { getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from '@solana/spl-token';
import idl from './idl/tokenizer.json' with { type: 'json' };
const METAPLEX_PROGRAM_ID = new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s');



function loadKeypair(path: string): Keypair {
  const secret = JSON.parse(readFileSync(path, 'utf8'));
  return Keypair.fromSecretKey(Uint8Array.from(secret));
}
//Insert path to keypairs below
const keypair1 = loadKeypair('');
const keypair2 = loadKeypair('');
const keypair3 = loadKeypair('');
const allOwners = [keypair1.publicKey, keypair2.publicKey, keypair3.publicKey];

const connection = new anchor.web3.Connection("https://api.testnet.solana.com", "confirmed");
const wallet = new anchor.Wallet(keypair1);
const provider = new anchor.AnchorProvider(connection, wallet, {
  commitment: 'confirmed',
});
anchor.setProvider(provider);

const programId = new PublicKey("ErCER1skeaeqzRzSaqpmAKEAZmGGdh9VRzNAXXWUXv8f");
const program = new anchor.Program(idl as anchor.Idl, programId, provider);
const splMintKeypair = Keypair.generate();
const METADATA_PROGRAM_ID = new PublicKey(MPL_TOKEN_METADATA_PROGRAM_ID);
const [metadataPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from('metadata'),
    METADATA_PROGRAM_ID.toBuffer(),
    splMintKeypair.publicKey.toBuffer(),
  ],
  METADATA_PROGRAM_ID
);
console.log("Metadata PDA:", metadataPda.toBase58());
console.log("Metadata Program ID:", METADATA_PROGRAM_ID.toBase58());

const [mintAuthority] = PublicKey.findProgramAddressSync(
  [Buffer.from("mint_auth"), splMintKeypair.publicKey.toBuffer()],
  program.programId
);
console.log("Mint Authority:", mintAuthority.toBase58());
console.log("SPL Mint Keypair:", splMintKeypair.publicKey.toBase58()); 

const [customMint] = PublicKey.findProgramAddressSync(
  [Buffer.from("custom_mint"), splMintKeypair.publicKey.toBuffer()],
  program.programId
);
console.log("Custom Mint PDA:", customMint.toBase58());

async function initialize() {
  console.log("Initializing token...");
  console.log("Accounts being sent:", {
    payer: keypair1.publicKey.toBase58(),
    customMint,
    splMint: splMintKeypair.publicKey.toBase58(),
    mintAuthority,
    metadata: metadataPda.toBase58(),
    tokenProgram: TOKEN_PROGRAM_ID.toBase58(),
    systemProgram: SystemProgram.programId.toBase58(),
    token_metadata_program: METAPLEX_PROGRAM_ID,
    sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY.toBase58(),
    rent: anchor.web3.SYSVAR_RENT_PUBKEY.toBase58(),
  });
  
  await program.methods
    .initialiseMint("Altarian42", "A42", new BN("10000000000000000"), allOwners, new BN(2))
    .accounts({
      payer: keypair1.publicKey,
      customMint,
      splMint: splMintKeypair.publicKey,
      mintAuthority,
      metadata: metadataPda,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      tokenMetadataProgram: METAPLEX_PROGRAM_ID,
      sysvarInstructions: SYSVAR_INSTRUCTIONS_PUBKEY,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .signers([keypair1, splMintKeypair])
    .rpc();

  console.log("Token initialized!");
  console.log("SPL Mint:", splMintKeypair.publicKey.toBase58());
  console.log("Custom Mint PDA:", customMint.toBase58());
}

initialize().catch((err) => {
  console.error("Error initializing mint:", err);
});
