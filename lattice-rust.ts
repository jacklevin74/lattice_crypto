import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LatticeRust } from "../target/types/lattice_rust";
import { BN } from "bn.js";

describe("lattice-rust", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.LatticeRust as Program<LatticeRust>;

  it("Simulates Lattice MPC with client-provided points for 50 players!", async () => {
    // Generate random x, y, z points for 50 players as BN (to simulate u64)
    const xPoints = Array.from({ length: 25 }, () => new BN(Math.floor(Math.random() * 10000)));
    const yPoints = Array.from({ length: 25 }, () => new BN(Math.floor(Math.random() * 10000)));
    const zPoints = Array.from({ length: 25 }, () => new BN(Math.floor(Math.random() * 10000)));

    // Log the generated points
    console.log("xPoints:", xPoints.map(p => p.toString()));
    console.log("yPoints:", yPoints.map(p => p.toString()));
    console.log("zPoints:", zPoints.map(p => p.toString()));

    // Call the simulateLatticeMpc method with the generated points
    const tx = await program.methods
      .simulateLatticeMpc(xPoints, yPoints, zPoints)
      .rpc();

    console.log("Your transaction signature", tx);
  });
});

