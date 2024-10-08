import {
    Connection,
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey,
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
    TransactionInstruction,
} from '@solana/web3.js';
import {readFileSync} from "fs";
import path from 'path';

const lo = require("buffer-layout");
// const BN = require("bn.js");



/**
 * Vars
 */

const SOLANA_NETWORK = "devnet";

let connection: Connection;
let programKeypair: Keypair;
let programId: PublicKey;

let dudiKeypair: Keypair;
let emoKeypair: Keypair;
let kiroKeypair: Keypair;
let nikoKeypair: Keypair;



/**
 * Helper functions.
 */

function createKeypairFromFile(path: string): Keypair {
    return Keypair.fromSecretKey(
        Buffer.from(JSON.parse(readFileSync(path, "utf-8")))
    )
}


/**
 * Here we are sending lamports using the Rust program we wrote.
 * So this looks familiar. We're just hitting our program with the proper instructions.
 */
async function sendLamports(from: Keypair, to: PublicKey, amount: number) {
    
    let data = Buffer.alloc(8) // 8 bytes
    // lo.ns64("value").encode(new BN(amount), data);
    lo.ns64("value").encode(amount, data);

    let ins = new TransactionInstruction({
        keys: [
            {pubkey: from.publicKey, isSigner: true, isWritable: false},
            {pubkey: to, isSigner: false, isWritable: true},
            {pubkey: SystemProgram.programId, isSigner: false, isWritable: false},
        ],
        programId: programId,
        data: data,
    });

    await sendAndConfirmTransaction(
        connection,
        new Transaction().add(ins),
        [from]
    );
}



/**
 * Main
 */

async function main() {
    
    connection = new Connection(
        `https://api.${SOLANA_NETWORK}.solana.com`, 'confirmed'
    );

    programKeypair = createKeypairFromFile(
        path.join(
            path.resolve(__dirname, '../_dist/program'), 
            'program-keypair.json'
        )
    );
    programId = programKeypair.publicKey;

    // Our sample members are Dudi, Emo, Kiro & Niko.
    dudiKeypair = createKeypairFromFile(__dirname + "/../accounts/dudi.json");
    emoKeypair = createKeypairFromFile(__dirname + "/../accounts/emo.json");
    kiroKeypair = createKeypairFromFile(__dirname + "/../accounts/kiro.json");
    nikoKeypair = createKeypairFromFile(__dirname + "/../accounts/niko.json");
    
    // We'll start by airdropping some lamports to Kiro & Niko.
    // await connection.confirmTransaction(
    //     await connection.requestAirdrop(
    //         kiroKeypair.publicKey,
    //         LAMPORTS_PER_SOL,
    //     )
    // );
    // await connection.confirmTransaction(
    //     await connection.requestAirdrop(
    //         nikoKeypair.publicKey,
    //         LAMPORTS_PER_SOL,
    //     )
    // );

    // Niko sends some SOL to Dudi.
    console.log("Dudi sends some SOL to Emo...");
    console.log(`   Dudi's public key: ${dudiKeypair.publicKey}`);
    console.log(`   Emo's public key: ${emoKeypair.publicKey}`);
    await sendLamports(dudiKeypair, emoKeypair.publicKey, 6250000);

    // Emo sends some SOL to Kiro.
    console.log("Emo sends some SOL to Kiro...");
    console.log(`   Emo's public key: ${emoKeypair.publicKey}`);
    console.log(`   Kiro's public key: ${kiroKeypair.publicKey}`);
    await sendLamports(emoKeypair, kiroKeypair.publicKey, 4150000);

    // Kiro sends some SOL over to Niko.
    console.log("Emo sends some SOL over to Niko...");
    console.log(`   Kiro's public key: ${kiroKeypair.publicKey}`);
    console.log(`   Niko's public key: ${nikoKeypair.publicKey}`);
    await sendLamports(kiroKeypair, nikoKeypair.publicKey, 2150000);
}


main().then(
    () => process.exit(),
    err => {
        console.error(err);
        process.exit(-1);
    },
  );
