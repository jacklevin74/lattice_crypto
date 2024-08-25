import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { LatticeRust } from "../target/types/lattice_rust";
import { BN } from "bn.js";
import * as crypto from "crypto";
import { exec } from "child_process"; // Node.js child_process module

describe("lattice-rust", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.LatticeRust as Program<LatticeRust>;
  const [statePda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("state")],
    program.programId
  );

  it("Initializes the program state", async () => {
    const tx = await program.rpc.initialize({
      accounts: {
        state: statePda,
        signer: program.provider.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
    });

    console.log("Program initialized with transaction signature:", tx);
  });

  it("Commits and reveals values for 10 players", async () => {
    const players: { x: BN, y: BN, z: BN, hash: Buffer }[] = [];

    for (let i = 0; i < 10; i++) {
      const x = new BN(Math.floor(Math.random() * 10000));
      const y = new BN(Math.floor(Math.random() * 10000));
      const z = new BN(Math.floor(Math.random() * 10000));

      const hash = crypto.createHash('sha256')
        .update(Buffer.concat([
          x.toArrayLike(Buffer, 'le', 8),
          y.toArrayLike(Buffer, 'le', 8),
          z.toArrayLike(Buffer, 'le', 8)
        ]))
        .digest();

      players.push({ x, y, z, hash });

      const commitTx = await program.rpc.commit([...hash], {
        accounts: {
          state: statePda,
          signer: program.provider.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
      });

      console.log(`Player ${i + 1} committed with transaction signature: ${commitTx}`);
    }

    // Ensure at least 6 blocks pass
    console.log("Waiting for at least 6 blocks to pass...");
    await new Promise(resolve => setTimeout(resolve, 3000)); // Adjust the delay as needed

    for (let i = 0; i < 10; i++) {
      const { x, y, z } = players[i];

      const revealTx = await program.rpc.reveal(i, x, y, z, {
        accounts: {
          state: statePda,
          signer: program.provider.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
      });

      console.log(`Player ${i + 1} revealed with transaction signature: ${revealTx}`);

      // For the last player, fetch the logs using solana confirm after 2 seconds delay
      if (i === 9) {
        console.log("Fetching program logs for Player 10...");

        setTimeout(() => {
          exec(`solana confirm -v ${revealTx} | grep "Program log"`, (error, stdout, stderr) => {
            if (error) {
              console.error(`Error fetching logs: ${error.message}`);
              return;
            }
            if (stderr) {
              console.error(`Error in stderr: ${stderr}`);
              return;
            }
            console.log("Program Logs:");
            console.log(stdout);
          });
        }, 2000); // Wait for 2 seconds before running the command
      }
    }
  });
});

