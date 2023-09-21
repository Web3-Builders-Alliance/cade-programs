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

  const board = [{
    kind: "tree",
    health: 1,
    dps: 1,
    position: 0
},

];
const budget = new anchor.BN(100);
  it("Create Map!", async () => {
    const tx = await program.methods.createMap(
      name,
      board,
      budget,

    )
    .accounts({
      user: anchorProvider.wallet.publicKey,
      map: mapPDA,
        systemProgram: web3.SystemProgram.programId,
    })
    .rpc();
    console.log("Your transaction signature", tx);
  });
  it("Get all Maps!", async () => {
    
    const map = await program.account.map.all()
    console.log(map)
  });
});
