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

## Installation and usage

git clone the program
anchor test 

  lattice-rust
Program initialized with transaction signature: Z1wnvY7jNW8AzLANArPnYEYuXrvN4qdHLd5Pb6ZSrx259MpHQSZsk8SZHrwNRqCYXkMFroYH6nQPRKMPTymDpKq
    ✔ Initializes the program state (3230ms)
Player 1 committed with transaction signature: 5yb6A1C5hxLhzKz5jAugUnorGHFw3H7KLvSpZxRR8MitKFNrRyXvEog4EwdnCMuxm9Mu2mXWxA1xqi1j7gDzv24T
Player 2 committed with transaction signature: 2T18nHpGXhUkrz7ksG83LUrToAuj35pbZiBjFJwmRiR5hqqCh67edCJ7aNsVe8aKmGcPTv4R1XBiQv4DuEZS83uQ
Player 3 committed with transaction signature: 4Cm86cGV9nhAd9JLpP8bkAda3uKvZKpcomGusxtiLMVzF366U2Tp96TpTF6DhTPkXQ4KqGdVFoQnwboJFkjTHoQy
Player 4 committed with transaction signature: 4BrUTGJpHYB8t2jAzeniCYHoneKincNGiwaQQ2RwRQVbaPpaAvDUSZ9kes6XbXFSZJXGaWgmNd4QAqJWpahEF5Cs
Player 5 committed with transaction signature: 3yPJeqpeoqVDvr5CuSVYVoGem2ZTidQ8vKLLcHUS1eMR1vpWVbD8nWxyYtLJ94CjjhSEUZUhBUhHHSJzJqBC2AYZ
Player 6 committed with transaction signature: 2v3RjD97G272vzFhATFFMCrXa884LQ1nsKEbWBEUA4aJSK4dwXioScV76U6XneaMhMp7kRko5S3rKEGwRwcFCQY
Player 7 committed with transaction signature: 5cSxSrJxTN2SkhaReRUSEfZWjbR9yN1v31riEbUYos5tQZyHQW7UgkJ6JRtpAsqH1QcFaPtR5DRJqsvW1ZVDHQmH
Player 8 committed with transaction signature: 19q3L9B2fNCrrtkrPFbBsyziKzxtnADuvQ7HKe4VMzojZB5RQYTZzzTWd1eCYagxVoYTJv7okhDMdvxwxdmaEqm
Player 9 committed with transaction signature: 5RWPQNyQK4CmqTzoxJpknxb2QE3yfwqzXeoRFYM75z3v5Tr7wBhhSoSbGNC24yEhCMxFCDxekU42ketZ3sU4jQ46
Player 10 committed with transaction signature: 2gDUQJTpumLuxyYPDEWrNxn5MdqEzgzpKrx9U2yzUUyDvy3heYL1HjGZvSW7YgAmBYuPxXTnA5nsdZgQJGZFft4V

Waiting for at least 6 blocks to pass...

Player 1 revealed with transaction signature: 5Urshgeuw9LLrWEHYUguP5ixfaVV9tNtiFPYBFRksogta87odZS1oRyCx3hFiKtWGsbjPZywNwzYZ4SEUXZyeY7A
Player 2 revealed with transaction signature: 2Pqo2HnmK4Zw3vQGnSCUoWVEFgSVrHwRwMdMYYiAbUxAitocTL6SXkFKq1oHvFTH4mUj9BzRLwTt6fqMv18Xpi6D
Player 3 revealed with transaction signature: 4F49qGKYYFrDKh1K2wR8HPRRMJ4SscqLPNnnLSPRtbeH9fDuLEbzSzqUN9v4NeKypvRbvj6MjJ3ZfFKDAyzxMKX6
Player 4 revealed with transaction signature: 4taBzXF1PdT634squ1tJ9NEYZzu2kPX3FRziSFJT2LrLFPLhLDr9AeyCmBDA4PTSS6TfTKKirmUDWbzVBtNC8gkZ
Player 5 revealed with transaction signature: EFd8bYFSkU69NjPnRh6zxuyYx9HVPieNHrdwSNtenvjCiRrXFPawgMEe4qJ4t19kBHSwayCaAaBzaByzz73xU9m
Player 6 revealed with transaction signature: dVfvMMnaXgryFjmNbnKxiMvQFfi6XBbeUNBkkatBAYdqfoyMR6c2PgCyX7EG1fP599UkhBBLw5ttmU7ZuZ8SeC4
Player 7 revealed with transaction signature: 474ba84H6DP25TmurFPvXYTEU7wLAXJLnscfU7cgnCDSsNHgVHcncf3zz4KDnbxbdhHgkQiGTgR2pLCFJ8dvzjAV
Player 8 revealed with transaction signature: 5ti4MHhDDWz15Ynzkvz1xxRHuedYgV4GPL6v3Wmkx3qs8idhuf6PnzXyhJpvXW4XmnRnaLuHERSDEfhsqs4KHnfP
Player 9 revealed with transaction signature: jqngKHeCReESqduqCtvyS5j6ngZcDATbMvzPKPNVvpYDYHebdKgVYYbTufF795PsFArKYWux5Zjs2Ln6pEYNQUG
Player 10 revealed with transaction signature: 2VZMqaffTtEVpPtwFwBVchaxCb8fcu1igHchBFGDkRYKU2UAAAFpKu4oLMVtGSHkdicaCsVCqxPhzXzEnwKVccWQ
Fetching program logs for Player 10...

Program Logs:
    Program log: Instruction: Reveal
    Program log: Computed hash for x=9302, y=6758, z=8218 is c0fd69b04d8ee29b7b2623a61a45e49cce911288ed74fb077ae6d558372684d8
    Program log: Player 9 has revealed their values.
    Program log: All players have revealed. Running the computation.
    Program log: SHA-256 Hash of total sum: 9900056937943159915
    Program log: Composite Point: x=7955, y=4843, z=6871
    Program log: Prime Point: x=7963, y=4861, z=6883
    Program log: The winner is Player 9, with a distance of 4571 to the prime point.
    Program log: The biggest loser is Player 0, with a distance of 17388 to the prime point.


