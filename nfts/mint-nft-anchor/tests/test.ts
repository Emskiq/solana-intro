import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
// import { ComputeBudgetProgram } from "@coral-xyz/anchor";
import { ComputeBudgetProgram } from '@solana/web3.js';
import { MintNft } from "../target/types/mint_nft";

describe("test", () => {
    // Configure the client to use the local cluster.
    const testNftTitle = "EMSKIQQQ";
    const testNftSymbol = "EMO";
    // const testNftUri = "Does not matter since we are attaching random photo from web (check lib.rs file)";

    const testNftUri = "https://raw.githubusercontent.com/Emskiq/solana-intro/refs/heads/master/nfts/assets/example.json";

    const provider = anchor.AnchorProvider.env();
    const wallet = provider.wallet as anchor.Wallet;

    anchor.setProvider(provider);

    const program = anchor.workspace.MintNft as Program<MintNft>;

    const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
        "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
    );

    const computeBudgetIx = ComputeBudgetProgram.requestUnits({
        units: 600_000, // Adjust units as needed (max 1,400,000)
        additionalFee: 0,
    });

    it("Mint!", async () => {

        // Derive the mint address and the associated token account address
        const mintKeypair: anchor.web3.Keypair = anchor.web3.Keypair.generate();
        const tokenAddress = await anchor.utils.token.associatedAddress({
            mint: mintKeypair.publicKey,
            owner: wallet.publicKey
        });
        console.log(`New token: ${mintKeypair.publicKey}`);

        // Derive the metadata address
        const metadataAddress = (await anchor.web3.PublicKey.findProgramAddress(
            [
                Buffer.from("metadata"),
                TOKEN_METADATA_PROGRAM_ID.toBuffer(),
                mintKeypair.publicKey.toBuffer(),
            ],
            TOKEN_METADATA_PROGRAM_ID
        ))[0];
        console.log("Metadata initialized");


        // Derive the master edition address
        const masterEditionAddress = (await anchor.web3.PublicKey.findProgramAddress(
            [
                Buffer.from("metadata"),
                TOKEN_METADATA_PROGRAM_ID.toBuffer(),
                mintKeypair.publicKey.toBuffer(),
                Buffer.from("edition"),
            ],
            TOKEN_METADATA_PROGRAM_ID
        ))[0];
        console.log("Master edition metadata initialized");

        // Transact with the "mint" function in our on-chain program
        //
        // The 'transaction' actually is just a function call to our program
        // that's the benefit of Anchor
        await program.methods.mint(
            testNftTitle, testNftSymbol, testNftUri
        )
        .accounts({
            masterEdition: masterEditionAddress,
            metadata: metadataAddress,
            mint: mintKeypair.publicKey,
            tokenAccount: tokenAddress,
            mintAuthority: wallet.publicKey,
            tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
        })
        // .preInstructions([computeBudgetIx])
        .signers([mintKeypair])
        .rpc();
    });
});
