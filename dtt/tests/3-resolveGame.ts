import * as anchor from "@coral-xyz/anchor";
import { Program, web3 } from "@coral-xyz/anchor";
import { Dtt } from "../target/types/dtt";

describe("Dtt", () => {
  const anchorProvider = anchor.AnchorProvider.env();
  anchor.setProvider(anchorProvider);
  const program = anchor.workspace.Dtt as Program<Dtt>;

  const name = "test";
  const [mapPDA] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from(name)],
    program.programId,
  )
  const game =  new web3.PublicKey("3GuACTeeimnwY6qjzw3KztwFGPgZ5RccBzfvzkDvi2J8")
  it("Resolve Game!", async () => {
    const tx = await program.methods.resolveGame()
    .accounts({
      user: anchorProvider.wallet.publicKey,
      map: mapPDA,
      game: game,
        systemProgram: web3.SystemProgram.programId,
    })
    .rpc()

    console.log("Your transaction signature", tx);
  });
  it("Get all games!", async () => {
    
    const map = await program.account.game.all()
    map.map((game) => {
      console.log(game)
      console.log(game.account.deploys)

    })
  });
});
