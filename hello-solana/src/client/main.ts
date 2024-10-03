import {
    Keypair,
    Connection,
    PublicKey,
    LAMPORTS_PER_SOL,
    TransactionInstruction,
    Transaction,
    sendAndConfirmTransaction,
    clusterApiUrl
} from '@solana/web3.js';
import fs from 'mz/fs';
import path from 'path';


/*
    Our keypair we used to create the on-chain Rust program
*/
const PROGRAM_KEYPAIR_PATH = path.join(
    path.resolve(__dirname, '../../dist/program'),
    'hello_solana-keypair.json'
);

function delay(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}


async function main() {

    console.log("Launching client...");

    /*
        Connect to Solana DEV net
    */
    const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

    /*
        Get our program's public key
    */
    const secretKeyString = await fs.readFile(PROGRAM_KEYPAIR_PATH, {encoding: 'utf8'});
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    const programKeypair = Keypair.fromSecretKey(secretKey);
    let programId: PublicKey = programKeypair.publicKey;



    /*
        Generate an account (keypair) to transact with our program
    */
    const triggerKeypair = Keypair.generate();

    console.log('Trigger Keypair Public Key:', triggerKeypair.publicKey.toBase58());

    // const airdropRequest = await connection.requestAirdrop(
    //     triggerKeypair.publicKey,
    //     LAMPORTS_PER_SOL,
    // );
    // await connection.confirmTransaction(airdropRequest);

    // Add a delay between requests to avoid rate limits
    await delay(10000); // 10 second delay between requests

    /*
        Conduct a transaction with our program
    */
    console.log('--Pinging Program ', programId.toBase58());
    const instruction = new TransactionInstruction({
        keys: [{pubkey: triggerKeypair.publicKey, isSigner: false, isWritable: true}],
        programId,
        data: Buffer.alloc(0),
    });
    await sendAndConfirmTransaction(
        connection,
        new Transaction().add(instruction),
        [triggerKeypair],
    );
}


main().then(
    () => process.exit(),
        err => {
        console.error(err);
        process.exit(-1);
    },
);
