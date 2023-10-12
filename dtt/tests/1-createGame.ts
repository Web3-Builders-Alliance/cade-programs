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
  const game =  anchor.web3.Keypair.generate();

  it("Create Game!", async () => {
    const tx = await program.methods.createGame()
    .accounts({
      user: anchorProvider.wallet.publicKey,
      map: mapPDA,
      game: game.publicKey,
        systemProgram: web3.SystemProgram.programId,
    })
    .signers([game])
    .rpc()

    console.log("Your transaction signature", tx);
  });
  it("Get all games!", async () => {
    
    const map = await program.account.game.all()
    console.log(map)
  });
});
