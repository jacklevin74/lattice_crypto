# Lattice-Rust: A Simulation of Lattice Shortest Vector Computation Using Commit-Reveal Scheme

## Overview

Lattice-Rust is a Solana-based smart contract that simulates a game where players compete to find the closest vector in a lattice. The game uses a commit-reveal scheme to ensure fairness, allowing players to generate the game setup themselves using their own random numbers. This simulation provides an example of how lattice-based cryptographic methods can be applied in a post-quantum context, using techniques that are resistant to attacks by quantum computers.

## Features

- **Commit-Reveal Scheme:** The game uses a commit-reveal process where players first commit to their chosen values by submitting a hash. After all players have committed, they reveal their original values. This prevents any player from cheating or changing their values after seeing others' submissions.
  
- **Multi-Party Computation (MPC):** The game setup is created through a decentralized process where players themselves generate the random numbers used in the game. MPC ensures that no single party controls the outcome, enhancing fairness and security.
  
- **Lattice-Based Cryptography:** The game leverages lattice-based cryptographic methods, which are known for their strong security properties, especially in the context of post-quantum cryptography. The lattice shortest vector problem (SVP) is used to determine the winner of the game.

## Commit-Reveal Scheme

The commit-reveal scheme is a two-phase protocol widely used in cryptographic applications to prevent cheating. In the **commit** phase, each player generates a random number and submits its hash, committing to that value without revealing it. In the **reveal** phase, after all players have committed, they reveal their original numbers. The system checks the revealed numbers against the original hashes to ensure integrity. This method ensures that players cannot change their values once they've seen others' commitments, maintaining fairness in the process.

## Multi-Party Computation (MPC)

MPC allows multiple parties to jointly compute a function over their inputs while keeping those inputs private. In this simulation, MPC is used to generate the game setup, where each player contributes random values. The random numbers provided by each player are used to create a composite game state, which ensures that no single player has control over the game setup. This decentralized approach is critical for maintaining fairness and trust in a competitive environment.

## Lattice-Based Cryptography

Lattice-based cryptography is a class of cryptographic algorithms that relies on the hardness of lattice problems, such as the Shortest Vector Problem (SVP). These problems are believed to be secure against quantum attacks, making them suitable for post-quantum cryptography. In this game, the SVP is used to determine the winner by finding which player's vector is closest to a target vector in the lattice. This approach demonstrates how lattice-based methods can be applied in practical cryptographic applications, especially in the context of emerging threats from quantum computing.

## How It Works

- **Initialization:** The program initializes by setting up a state that will hold player data and the current phase of the game.

- **Commit Phase:** Each player generates random values (x, y, z) and commits to them by submitting their SHA-256 hash.

- **Reveal Phase:** After a set number of blocks, the players reveal their original values. The program verifies the values against the hashes.

- **Winner Determination:** Once all values are revealed and verified, the program computes the composite and prime points in the lattice. The player whose vector is closest to the prime point is declared the winner.

## How to run:

anchor test 


