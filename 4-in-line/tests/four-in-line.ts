import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { FourInLine } from "../target/types/four_in_line";
import { PublicKey, Keypair } from "@solana/web3.js"
import * as web3 from "@solana/web3.js"

describe("four-in-line", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.FourInLine as Program<FourInLine>;
  const payer = (program.provider as anchor.AnchorProvider).wallet;
  const player2 = new PublicKey("EjPpXXDykPawauyZHsBMtxGwG7K4iFmxdvB6ockM56ZN")
  const game = new Keypair()
  

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.createGame(
      [
        payer.publicKey,
        player2
      ],
      ""
    )
      .accounts({
        payer: payer.publicKey,
        game: game.publicKey,
      })
      .signers([game])
      .rpc({
        skipPreflight: true,
      }).catch((err) => {
        console.log(err)
        });
    console.log("Your transaction signature", tx);
    const games = await program.account.game.all()
    console.log(games)
  });
});
