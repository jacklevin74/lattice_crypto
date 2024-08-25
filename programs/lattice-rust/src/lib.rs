use anchor_lang::prelude::*;
use sha2::{Digest, Sha256};

declare_id!("Gqhcmy6G9JYiS31b3D9FkBmXuyYRebzYG4oY2AqGUwqX");

#[program]
pub mod lattice_rust {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.players = Vec::new();
        state.phase = GamePhase::Commit;
        state.last_commit_block = 0;
        Ok(())
    }

    pub fn add_player(ctx: Context<AddPlayer>, x: u64, y: u64, z: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;

        require!(state.phase == GamePhase::Commit, CustomError::InvalidPhase);

        let player_id = state.players.len() as u8; // Calculate the player ID first
        state.players.push(Player {
            id: player_id,
            x: Some(x),
            y: Some(y),
            z: Some(z),
            hash: None,
        });

        if state.players.len() == 10 {
            compute_winner(state)?;
        }

        Ok(())
    }

    pub fn commit(ctx: Context<Commit>, hash: [u8; 32]) -> Result<()> {
        let state = &mut ctx.accounts.state;

        require!(state.phase == GamePhase::Commit, CustomError::InvalidPhase);

        let player_id = state.players.len() as u8;
        state.players.push(Player {
            id: player_id,
            x: None,
            y: None,
            z: None,
            hash: Some(hash),
        });

        state.last_commit_block = Clock::get()?.slot;

        if state.players.len() == 10 {
            state.phase = GamePhase::Reveal;
        }

        Ok(())
    }

    pub fn reveal(ctx: Context<Reveal>, player_id: u8, x: u64, y: u64, z: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;

        require!(state.phase == GamePhase::Reveal, CustomError::InvalidPhase);
        require!(
            Clock::get()?.slot >= state.last_commit_block + 6,
            CustomError::BlocksNotPassed
        );

        // Find the player by their ID
        let player = state
            .players
            .iter_mut()
            .find(|p| p.id == player_id)
            .ok_or(CustomError::PlayerNotFound)?;

        let computed_hash = compute_hash(x, y, z);
        require!(player.hash.unwrap() == computed_hash, CustomError::HashMismatch);

        player.x = Some(x);
        player.y = Some(y);
        player.z = Some(z);

        msg!("Player {} has revealed their values.", player_id);

        if state
            .players
            .iter()
            .all(|p| p.x.is_some() && p.y.is_some() && p.z.is_some())
        {
            msg!("All players have revealed. Running the computation.");
            compute_winner(state)?;
            state.phase = GamePhase::Commit; // Reset for the next game
        }

        Ok(())
    }
}

// Helper function to compute the SHA-256 hash of x, y, and z values
fn compute_hash(x: u64, y: u64, z: u64) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(&x.to_le_bytes());
    hasher.update(&y.to_le_bytes());
    hasher.update(&z.to_le_bytes());
    let result = hasher.finalize();
    let mut hash_bytes = [0u8; 32];
    hash_bytes.copy_from_slice(&result);

    msg!("Computed hash for x={}, y={}, z={} is {}", x, y, z, hex::encode(hash_bytes));
    hash_bytes
}


// Helper function to compute the winner
fn compute_winner(state: &mut ProgramState) -> Result<()> {
    let mut min_x = u64::MAX;
    let mut max_x = u64::MIN;
    let mut min_y = u64::MAX;
    let mut max_y = u64::MIN;
    let mut min_z = u64::MAX;
    let mut max_z = u64::MIN;
    let mut total_sum = 0u64;

    for player in &state.players {
        if let (Some(x), Some(y), Some(z)) = (player.x, player.y, player.z) {
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            max_y = max_y.max(y);
            min_z = min_z.min(z);
            max_z = max_z.max(z);
            total_sum = total_sum.wrapping_add(x.wrapping_add(y).wrapping_add(z));
        }
    }

    let hash = hash_sum(total_sum);
    let composite_point = LatticePoint {
        x: min_x + (hash % (max_x - min_x)),
        y: min_y + (hash % (max_y - min_y)),
        z: min_z + (hash % (max_z - min_z)),
    };

    msg!("SHA-256 Hash of total sum: {}", hash);
    msg!("Composite Point: x={}, y={}, z={}", composite_point.x, composite_point.y, composite_point.z);

    let prime_point = LatticePoint {
        x: next_prime(composite_point.x),
        y: next_prime(composite_point.y),
        z: next_prime(composite_point.z),
    };

    msg!("Prime Point: x={}, y={}, z={}", prime_point.x, prime_point.y, prime_point.z);

    let mut closest_distance = None;
    let mut farthest_distance = None;
    let mut winner_id = 0;
    let mut loser_id = 0;

    for player in &state.players {
        if let (Some(x), Some(y), Some(z)) = (player.x, player.y, player.z) {
            let distance = calculate_distance(x, prime_point.x)
                + calculate_distance(y, prime_point.y)
                + calculate_distance(z, prime_point.z);

            match closest_distance {
                None => {
                    closest_distance = Some(distance);
                    winner_id = player.id;
                }
                Some(ref mut closest) => {
                    if distance < *closest {
                        *closest = distance;
                        winner_id = player.id;
                    }
                }
            }

            match farthest_distance {
                None => {
                    farthest_distance = Some(distance);
                    loser_id = player.id;
                }
                Some(ref mut farthest) => {
                    if distance > *farthest {
                        *farthest = distance;
                        loser_id = player.id;
                    }
                }
            }
        }
    }

    msg!("The winner is Player {}, with a distance of {} to the prime point.", winner_id, closest_distance.unwrap());
    msg!("The biggest loser is Player {}, with a distance of {} to the prime point.", loser_id, farthest_distance.unwrap());

    state.players.clear(); // Reset for the next game
    Ok(())
}

// Helper function to calculate the absolute value of the difference between two u64s
fn calculate_distance(a: u64, b: u64) -> u64 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

// Helper function to hash the sum of values and return a u64
fn hash_sum(sum: u64) -> u64 {
    let hash = Sha256::digest(&sum.to_be_bytes());
    let mut result = [0u8; 8];
    result.copy_from_slice(&hash[0..8]);
    u64::from_be_bytes(result)
}

// Function to find the next prime number after a given number
fn next_prime(mut num: u64) -> u64 {
    num += 1;
    while !is_prime(num) {
        num += 1;
    }
    num
}

// Simple primality test
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
#[derive(PartialEq)]
pub enum GamePhase {
    Commit,
    Reveal,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Player {
    pub id: u8,
    pub x: Option<u64>,
    pub y: Option<u64>,
    pub z: Option<u64>,
    pub hash: Option<[u8; 32]>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct LatticePoint {
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

#[account]
pub struct ProgramState {
    pub players: Vec<Player>,
    pub phase: GamePhase,
    pub last_commit_block: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init_if_needed, payer = signer, space = 8 + 1024, seeds = [b"state".as_ref()], bump)]
    pub state: Account<'info, ProgramState>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddPlayer<'info> {
    #[account(mut, seeds = [b"state".as_ref()], bump)]
    pub state: Account<'info, ProgramState>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Commit<'info> {
    #[account(mut, seeds = [b"state".as_ref()], bump)]
    pub state: Account<'info, ProgramState>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Reveal<'info> {
    #[account(mut, seeds = [b"state".as_ref()], bump)]
    pub state: Account<'info, ProgramState>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum CustomError {
    #[msg("The number of players is not equal to 10.")]
    InvalidPlayerCount,
    #[msg("Invalid phase for this operation.")]
    InvalidPhase,
    #[msg("Hash mismatch detected.")]
    HashMismatch,
    #[msg("Player not found.")]
    PlayerNotFound,
    #[msg("Not enough blocks have passed since the last commit.")]
    BlocksNotPassed,
}

