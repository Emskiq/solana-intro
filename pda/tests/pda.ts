import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Pda } from "../target/types/pda";

function shortKey(key: anchor.web3.PublicKey) {
    return key.toString().substring(0, 8);
}

describe("pda", () => {
    // Configure the client to use the local cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.Pda as Program<Pda>;
    const wallet = provider.wallet as anchor.Wallet;


    async function generateKeypair() {
        let keypair = anchor.web3.Keypair.generate();
        // Let's see if this works... with vanilla Solana JS SDK it didn't
        // --- NO FUCK... IT DOES NOT...
        await provider.connection.requestAirdrop(
            keypair.publicKey,
            2 * anchor.web3.LAMPORTS_PE_SOL
        );
        await new Promise( resolve => setTimeout(resolve, 3 * 1000) ); // Sleep 3s
        return keypair;
    }

    async function derivePda(color: string, pubkey: anchor.web3.PublicKey) {
        let [pda, _] = await anchor.web3.PublicKey.findProgramAddressSync(
            [
                pubkey.toBuffer(),
                Buffer.from("_"),
                Buffer.from(color),
            ],
            program.programId
        );
        return pda;
    }

    async function createLedgerTest(color: string, pda: anchor.web3.PublicKey, wallet: anchor.web3.Keypair) {
        await program.methods.createLedger(
            color
        )
        .accounts({
            ledgerAccount: pda,
            wallet: wallet.publicKey,
        })
        .rpc();
    }

    async function modifyLedgerTest(color: string, balance: number, wallet: anchor.web3.Keypair) {
        console.log("--------------------------------------------------");
        let data;
        let pda = await derivePda(color, wallet.publicKey);

        console.log(`Checking if account ${shortKey(pda)} exists for color: ${color}...`);
        try {

            data = await program.account.ledger.fetch(pda);
            console.log("It does.");

        } catch (e) {

            console.log("It does NOT. Creating...");
            await createLedgerTest(color, pda, wallet);
            data = await program.account.ledger.fetch(pda);
        };

        console.log("Success.");
        console.log("Data:")
        console.log(`    Color: ${data.color}   Balance: ${data.balance}`);
        console.log(`Modifying balance of ${data.color} from ${data.balance} to ${balance}`);

        await program.methods.modifyLedger(balance)
        .accounts({
            ledgerAccount: pda,
            wallet: wallet.publicKey,
        })
        .rpc();

        data = await program.account.ledger.fetch(pda);
        console.log("New Data:")
        console.log(`    Color: ${data.color}   Balance: ${data.balance}`);
        console.log("Success.");
    }

    it("PDA Tests", async () => {
        // const testKeypair1 = await generateKeypair();
        await modifyLedgerTest("red", 2, wallet);
        await modifyLedgerTest("red", 4, wallet);
        await modifyLedgerTest("blue", 2, wallet);

        // const testKeypair2 = await generateKeypair();
        await modifyLedgerTest("red", 3, wallet);
        await modifyLedgerTest("green", 3, wallet);
    });
});
