// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { SolanaNftAnchor } from "../target/types/solana_nft_anchor";

// describe("solana-nft-anchor", () => {
//   // Configure the client to use the local cluster.
//   anchor.setProvider(anchor.AnchorProvider.env());

//   const program = anchor.workspace.SolanaNftAnchor as Program<SolanaNftAnchor>;

//   it("Is initialized!", async () => {
//     // Add your test here.
//     const tx = await program.methods.initialize().rpc();
//     console.log("Your transaction signature", tx);
//   });
// });

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaNftAnchor} from "../target/types/solana_nft_anchor";
import { walletAdapterIdentity } from "@metaplex-foundation/umi-signer-wallet-adapters";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import {
    findMasterEditionPda,
    findMetadataPda,
    mplTokenMetadata,
    MPL_TOKEN_METADATA_PROGRAM_ID,
} from "@metaplex-foundation/mpl-token-metadata";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { publicKey } from "@metaplex-foundation/umi";

import {
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

describe("solana-nft-anchor", async () => {
    // Configured the client to use the devnet cluster.
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    console.log("provider");

    const idl= JSON.parse(
        require("fs").readFileSync("./target/idl/solana_nft_anchor.json", "utf8")
    )
    const programId =  new anchor.web3.PublicKey("7B1LRb9KW4K6nfDR7K8stLFDM78UbReofX4aMc28WBNX");
    console.log("programId", programId);
    
    



    const signer = provider.wallet;

    console.log("signer", signer);
    const program = anchor.workspace.SolanaNftAnchor as Program<SolanaNftAnchor>;
    console.log("program", program);
    const umi = createUmi("https://api.devnet.solana.com")
        .use(walletAdapterIdentity(signer))
        .use(mplTokenMetadata());

    const mint = anchor.web3.Keypair.generate();

    // Derive the associated token address account for the mint
    const associatedTokenAccount = await getAssociatedTokenAddress(
        mint.publicKey,
        signer.publicKey
    );

    // derive the metadata account
    let metadataAccount = findMetadataPda(umi, {
        mint: publicKey(mint.publicKey),
    })[0];


    const metadata = {
        name: "Bprime's groot",
        symbol: "BPG",
        uri: "https://raw.githubusercontent.com/687c/solana-nft-native-client/main/metadata.json",
    };

    it("mints nft!", async () => {
        const tx = await program.methods
            .initNft(metadata.name, metadata.symbol, metadata.uri)
            .accounts({
                signer: provider.publicKey,
                mint: mint.publicKey,
                associatedTokenAccount,
                metadataAccount,
             
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                tokenMetadataProgram: MPL_TOKEN_METADATA_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
            })
            .signers([mint])
            .rpc();

        console.log(
            `mint nft tx: https://explorer.solana.com/tx/${tx}?cluster=devnet`
        );
        console.log(
            `minted nft: https://explorer.solana.com/address/${mint.publicKey}?cluster=devnet`
        );
    });
});