use anchor_lang::prelude::*;
use sha2::{Digest, Sha256};

declare_id!("Gqhcmy6G9JYiS31b3D9FkBmXuyYRebzYG4oY2AqGUwqX");

#[program]
mod lattice_rust {
    use super::*;

    pub fn simulate_lattice_mpc(_ctx: Context<SimulateLatticeMpc>, x_points: Vec<u64>, y_points: Vec<u64>, z_points: Vec<u64>) -> Result<()> {
        require!(x_points.len() == 25, CustomError::InvalidPlayerCount);
        require!(y_points.len() == 25, CustomError::InvalidPlayerCount);
        require!(z_points.len() == 25, CustomError::InvalidPlayerCount);

        let mut players = Vec::new();
        let mut min_x = u64::MAX;
        let mut max_x = u64::MIN;
        let mut min_y = u64::MAX;
        let mut max_y = u64::MIN;
        let mut min_z = u64::MAX;
        let mut max_z = u64::MIN;

        // Sum of all x, y, z values for hashing
        let mut total_sum = 0u64;

        // Add players and their random points from the client
        for i in 0..25 {
            let x = x_points[i];
            let y = y_points[i];
            let z = z_points[i];

            players.push(Player {
                id: i as u8,
                x,
                y,
                z,
            });

            // Update min and max values for x, y, z
            if x < min_x {
                min_x = x;
            }
            if x > max_x {
                max_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if y > max_y {
                max_y = y;
            }
            if z < min_z {
                min_z = z;
            }
            if z > max_z {
                max_z = z;
            }

            // Add to total sum for hash
            total_sum = total_sum.wrapping_add(x.wrapping_add(y).wrapping_add(z));
        }

        // Calculate the composite point based on the sum hash
        let hash = hash_sum(total_sum);
        msg!("SHA-256 Hash of total sum: {}", hash);

        let composite_point = LatticePoint {
            x: min_x + (hash % (max_x - min_x)),
            y: min_y + (hash % (max_y - min_y)),
            z: min_z + (hash % (max_z - min_z)),
        };

        msg!("Composite Point: x={}, y={}, z={}", composite_point.x, composite_point.y, composite_point.z);

        // Calculate the prime point
        let prime_point = LatticePoint {
            x: next_prime(composite_point.x),
            y: next_prime(composite_point.y),
            z: next_prime(composite_point.z),
        };

        msg!("Prime Point: x={}, y={}, z={}", prime_point.x, prime_point.y, prime_point.z);

        // Determine the winner and the biggest loser based on the shortest and longest distance
        let mut closest_distance = None;
        let mut farthest_distance = None;
        let mut winner_id = 0;
        let mut loser_id = 0;

        for player in &players {
            let distance = calculate_distance(player.x, prime_point.x)
                + calculate_distance(player.y, prime_point.y)
                + calculate_distance(player.z, prime_point.z);

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

        msg!("The winner is Player {}, with a distance of {} to the prime point.", winner_id, closest_distance.unwrap());
        msg!("The biggest loser is Player {}, with a distance of {} to the prime point.", loser_id, farthest_distance.unwrap());

        Ok(())
    }
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

#[derive(Debug, Clone)]
pub struct Player {
    pub id: u8,
    pub x: u64,
    pub y: u64,
    pub z: u64,
}

#[derive(Debug, Clone)]
pub struct LatticePoint {
    pub x: u64,
    pub y: u64,
    pub z: u64,
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

#[derive(Accounts)]
pub struct SimulateLatticeMpc<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum CustomError {
    #[msg("The number of players is not equal to 3.")]
    InvalidPlayerCount,
}

