import * as anchor from "@coral-xyz/anchor";
// ** Comment this to use solpg imported IDL **
import {
    createKeypairFromFile,
} from './util';
import { MintNft } from "../target/types/mint_nft";


describe("sell-nft", async () => {


    const provider = anchor.AnchorProvider.env()
    const wallet = provider.wallet as anchor.Wallet;
    anchor.setProvider(provider);

    // ** Un-comment this to use solpg imported IDL **
    // const program = new anchor.Program(
    //   require("../solpg/idl.json"), 
    //   new anchor.web3.PublicKey("H2UJjAQTuVJYhaBhh6GD2KaprLBTp1vhP2aaHioya5NM"),
    // );
    // ** Comment this to use solpg imported IDL **
    const program = anchor.workspace.MintNft as anchor.Program<MintNft>;


    it("Sell!", async () => {

        // Testing constants

        const saleAmount = 1 * anchor.web3.LAMPORTS_PER_SOL;
        const mint: anchor.web3.PublicKey = new anchor.web3.PublicKey(
            "DdXA1TxN9MTWN5tKSszfWxdM9kVpfagA9TksYer1wvh7"
        );
        const buyer: anchor.web3.Keypair = await createKeypairFromFile(__dirname + "/keypairs/buyer.json");
        console.log(`Buyer public key: ${buyer.publicKey}`);

        // Derive the associated token account address for owner & buyer

        const sellerTokenAddress = await anchor.utils.token.associatedAddress({
            mint: mint,
            owner: wallet.publicKey
        });
        const buyerTokenAddress = await anchor.utils.token.associatedAddress({
            mint: mint,
            owner: buyer.publicKey,
        });
        console.log(`Request to sell NFT: ${mint} for ${saleAmount} lamports.`);
        console.log(`Owner's Token Address: ${sellerTokenAddress}`);
        console.log(`Buyer's Token Address: ${buyerTokenAddress}`);

        // Transact with the "sell" function in our on-chain program

        await program.methods.sell(
            new anchor.BN(saleAmount)
        )
        .accounts({
            mint: mint,
            sellerAuthority: wallet.publicKey,
            sellerTokenAccount: sellerTokenAddress,
            buyerAuthority: buyer.publicKey,
            buyerTokenAccount: buyerTokenAddress,
        })
        .signers([buyer])
        .rpc();
    });
});
