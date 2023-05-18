use rand::distributions::WeightedIndex;
use rand::prelude::*;

fn utility(a: usize, b: usize) -> i32 {
    if a == 0 {
        if b == 0 {
            return 0;
        } else if b == 1 {
            return -1;
        } else {
            return 1;
        }
    } else if a == 1 {
        if b == 0 {
            return 1;
        } else if b == 1 {
            return 0;
        } else {
            return -1;
        }
    } else {
        if b == 0 {
            return -1;
        } else if b == 1 {
            return 1;
        } else {
            return 0;
        }
    }
}

fn main() {
    // Define cumulative_regrets[p][a] as the cumulative regret of player `p` for not having chosen
    // action `a` where
    // - `a` == 0 => Rock
    // - `a` == 1 => Paper
    // - `a` == 2 => Scissors
    let mut cumulative_regrets = [[0_i32; 3]; 2];
    let mut strategy_profile_sum = [[0_f32; 3]; 2];
    let iterations: i32 = 500_000;
    for i in 0..iterations {
        // Compute a regret-matching strategy profile.
        // If all regrets are non-positive, use a uniform random strategy.
        // A strategy profile for player `p`, strategy_profile[p], is
        // the probability distribution of player `p` choosing a certain action.
        let mut strategy_profile = [[1_f32 / 3_f32; 3]; 2];
        for p in 0..=1 {
            let cumulative_regrets_sum: i32 = cumulative_regrets[p]
                .iter()
                .map(|&x| {
                    if x > 0 {
                        return x;
                    }
                    0
                })
                .sum();
            if cumulative_regrets_sum == 0 {
                continue;
            }
            for a in 0..=2 {
                if cumulative_regrets[p][a] > 0 {
                    strategy_profile[p][a] =
                        cumulative_regrets[p][a] as f32 / cumulative_regrets_sum as f32;
                } else {
                    strategy_profile[p][a] = 0_f32;
                }
            }
        }

        // Add the strategy profile to the strategy profile sum.
        for p in 0..=1 {
            for a in 0..=2 {
                strategy_profile_sum[p][a] += strategy_profile[p][a];
            }
        }

        // Select each player's action profile according to the strategy profile.
        // chosen_actions[p] is the action chosen by the player `p` given the probability
        // distribution, `strategy_profile[p]`.
        let mut chosen_actions = [0_usize; 2];
        for p in 0..=1 {
            chosen_actions[p] = WeightedIndex::new(&strategy_profile[p])
                .unwrap()
                .sample(&mut thread_rng());
        }

        // Compute player regrets.
        let mut regrets = [[0; 3]; 2];
        for p in 0..=1 {
            let current_utility = utility(chosen_actions[p], chosen_actions[p ^ 1]);
            for a in 0..=2 {
                regrets[p][a] = utility(a, chosen_actions[p ^ 1]) - current_utility;
            }
        }

        // Add player regrets to player cumulative regrets.
        for p in 0..=1 {
            for a in 0..=2 {
                cumulative_regrets[p][a] += regrets[p][a];
            }
        }

        // Print the average strategy profile.
        if (i + 1) % 100_000 == 0 {
            println!("Iteration {}", i + 1);
            let mut average_strategy_profile = strategy_profile_sum.clone();
            for p in 0..=1 {
                for a in 0..=2 {
                    average_strategy_profile[p][a] /= (i + 1) as f32;
                }
            }
            println!(
                "Average strategy profile at iteration {}: {:#?}",
                i + 1,
                average_strategy_profile
            );
        }
    }
}
