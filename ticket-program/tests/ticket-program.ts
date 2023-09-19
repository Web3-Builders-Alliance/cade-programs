import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TicketProgram } from "../target/types/ticket_program";

describe("ticket-program", () => {
  // Configure the client to use the local cluster.
  const METADATA_SEED = "metadata";
    const TOKEN_METADATA_PROGRAM_ID = new web3.PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

    // Constants from our program
    const MINT_SEED = "mint";
  
    // Data for our tests
    const payer = pg.wallet.publicKey;
    const metadata = {
      name: "Cade",
      symbol: "CDX",
      uri: "https://pan5exjah3p3wqjeujj4kylo4vgt5znvwrt77lnf5gcmm6tadsea.arweave.net/eBvSXSA-37tBJKJTxWFu5U0-5bW0Z_-tpemExnpgHIg",
      decimals: 9,
    };
    const mintAmount = 10;
    const [mint] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(MINT_SEED)],
      pg.PROGRAM_ID
    );

    const [metadataAddress] = web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from(METADATA_SEED),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        mint.toBuffer(),
      ],
      TOKEN_METADATA_PROGRAM_ID
    );

    // Test init token
    it("init cade", async () => {
      const info = await pg.connection.getAccountInfo(mint);
      if (info) {
        return; // Do not attempt to initialize if already initialized
      }
      console.log("  Mint not found. Attempting to initialize.");
   
      const context = {
        metadata: metadataAddress,
        mint,
        payer,
        rent: web3.SYSVAR_RENT_PUBKEY,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        tokenMetadataProgram: TOKEN_METADATA_PROGRAM_ID,
      };
  
      const txHash = await pg.program.methods
        .initCade(metadata)
        .accounts(context)
        .rpc();
  
      await pg.connection.confirmTransaction(txHash, 'finalized');
      console.log(`  https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
      const newInfo = await pg.connection.getAccountInfo(mint);
      assert(newInfo, "  Mint should be initialized.");
    });

     // Test mint tokens
     it("mint cade", async () => {
      const destination = await anchor.utils.token.associatedAddress({
        mint: mint,
        owner: payer,
      });
  
      let initialBalance: number;
      try {
        const balance = (await pg.connection.getTokenAccountBalance(destination))
        initialBalance = balance.value.uiAmount;
      } catch {
        // Token account not yet initiated has 0 balance
        initialBalance = 0;
      } 
      
      const context = {
        mint,
        destination,
        payer,
        rent: web3.SYSVAR_RENT_PUBKEY,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        associatedTokenProgram: anchor.utils.token.ASSOCIATED_PROGRAM_ID,
      };
  
      const txHash = await pg.program.methods
        .mintCade(new BN(mintAmount * 10 ** metadata.decimals))
        .accounts(context)
        .rpc();
      await pg.connection.confirmTransaction(txHash);
      console.log(`  https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
  
     
      });

      it("transfer", async () => {
        const fromKp = pg.wallet.keypair;
        const to = new PublicKey("2JSg1MdNqRg9z4RP7yiE2NV86fux2BNtF3pSDjhoi767s")
    
        const mint = new PublicKey("CHwKjh5GbbtDJDqnGKLwv1nZTJKuGAqhZkpjcwxqkyz3")
        
        const fromAta = await createAssociatedTokenAccount(
          pg.program.provider.connection,
          pg.wallet.keypair,
          mint,
          fromKp.publicKey
        );
    
         const toAta = await createAssociatedTokenAccount(
          pg.program.provider.connection,
          pg.wallet.keypair,
          mint,
          to
        );
    
         const transferAmount = new BN(2);
    
         const txHash = await pg.program.methods
          .transferSplTokens(transferAmount)
          .accounts({
            from: fromKp.publicKey,
            fromAta: fromAta,
            toAta: toAta,
            tokenProgram: TOKEN_PROGRAM_ID,
          })
          .signers([pg.wallet.keypair, fromKp])
          .rpc();
    
          console.log(`https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
          await pg.connection.confirmTransaction(txHash, "finalized");
      });
});
