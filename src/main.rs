use std::time::Instant;

use num_bigint::{BigInt, BigUint, ToBigInt};

#[cfg(test)]
mod test;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let faces: usize = args
        .get(1)
        .expect("Please provide faces as first argument")
        .parse()
        .expect("Faces must be a valid number");

    let max_iterations: usize = args
        .get(2)
        .expect("Please provide max_iterations as second argument")
        .parse()
        .expect("Max Iterations must be a valid number");

    let answer_decimals: usize = args
        .get(3)
        .unwrap_or(&"10".to_string())
        .parse()
        .expect("Decimal Accuracy must be a valid number");

    assert_eq!(faces % 2, 0, "Faces must be even");
    assert!(
        faces >= 4,
        "Faces must be at least 4 to have a valid starting position"
    );

    let start = Instant::now();

    println!(
        "Solving for total outcomes of a clock puzzle with {} faces, doing {} iterations",
        faces, max_iterations
    );

    let results = solve_iterative(faces, max_iterations);

    results.print_results(faces, max_iterations, answer_decimals);

    let elapsed = start.elapsed();
    println!("Finished in {:?}", elapsed);
}

fn solve_iterative(faces: usize, max_iterations: usize) -> Results {
    let mut wins = BigUint::ZERO;
    let mut losses = BigUint::ZERO;

    // arrays to hold the number of ways to reach each position
    // the seen left and right arrays indicate whether we've seen
    // the face immediately adjacent to the target face
    // note that none seen could be 2 faces smaller and left and right
    // could be 1 face smaller but this size alligns them much easier
    let seen_size = faces - 1;
    let mut seen_none = vec![BigUint::ZERO; seen_size];
    let mut seen_left = vec![BigUint::ZERO; seen_size];
    let mut seen_right = vec![BigUint::ZERO; seen_size];

    // the array is shifted such that the middle index
    // is the starting face (12 o'clock)
    let shift = (faces - 2) / 2;

    seen_none[shift] = BigUint::from(1u8);

    for current_iteration in 0..max_iterations {
        let mut next_seen_none = vec![BigUint::ZERO; seen_size];
        let mut next_seen_left = vec![BigUint::ZERO; seen_size];
        let mut next_seen_right = vec![BigUint::ZERO; seen_size];

        // earlier rounds have more weight because
        // the possible outcomes double each iteration
        let weight = max_iterations - current_iteration - 1;

        for (i, n) in seen_none.iter().enumerate() {
            if *n == BigUint::ZERO {
                continue;
            }

            // edge case for a size of 3 (4 faces) where the
            // second (i = 1) and second to last (i = n - 1) index are the same
            if seen_size < 4 && i == 1 {
                next_seen_left[i - 1] += n;
                next_seen_right[i + 1] += n;
                continue;
            }

            // if the next iteration would land on an edge, it's moved
            // to either the left or right seen array
            if i == 1 {
                next_seen_none[i + 1] += n;
                next_seen_left[i - 1] += n;
                continue;
            }
            if i == seen_size - 2 {
                next_seen_none[i - 1] += n;
                next_seen_right[i + 1] += n;
                continue;
            }

            next_seen_none[i - 1] += n;
            next_seen_none[i + 1] += n;
        }
        for (i, n) in seen_left.iter().enumerate() {
            if *n == BigUint::ZERO {
                continue;
            }

            // if we're at the left edge and we've only seen the left
            // we lose because this means that we havn't seen all other faces
            // before reaching the target face
            if i == 0 {
                losses += n << weight;
                next_seen_left[i + 1] += n;
                continue;
            }

            // if the next iteration would reach the right edge
            // this means we've seen both edges, so we win
            if i == seen_size - 2 {
                wins += n << weight;
                next_seen_left[i - 1] += n;
                continue;
            }

            next_seen_left[i - 1] += n;
            next_seen_left[i + 1] += n;
        }
        for (i, n) in seen_right.iter().enumerate() {
            if *n == BigUint::ZERO {
                continue;
            }

            // if we're at the right edge and we've only seen the right
            // we lose because this means that we havn't seen all other faces
            // before reaching the target face
            if i == seen_size - 1 {
                losses += n << weight;
                next_seen_right[i - 1] += n;
                continue;
            }

            // if the next iteration would reach the left edge
            // this means we've seen both edges, so we win
            if i == 1 {
                wins += n << weight;
                next_seen_right[i + 1] += n;
                continue;
            }
            next_seen_right[i - 1] += n;
            next_seen_right[i + 1] += n;
        }

        seen_none = next_seen_none;
        seen_left = next_seen_left;
        seen_right = next_seen_right;
    }

    let ties = seen_none.iter().sum::<BigUint>()
        + seen_left.iter().sum::<BigUint>()
        + seen_right.iter().sum::<BigUint>();

    Results { wins, losses, ties }
}

struct Results {
    wins: BigUint,
    losses: BigUint,
    ties: BigUint,
}

impl Results {
    fn print_results(&self, faces: usize, max_iterations: usize, answer_decimals: usize) {
        println!(
            "Results for faces: {}, iterations: {}",
            faces, max_iterations
        );
        let total = &self.wins + &self.losses + &self.ties;
        println!(
            "{} <= p <= {}",
            format_rational(
                &self.wins.to_bigint().unwrap(),
                &total.to_bigint().unwrap(),
                answer_decimals
            ),
            format_rational(
                &(&self.wins + &self.ties).to_bigint().unwrap(),
                &total.to_bigint().unwrap(),
                answer_decimals
            )
        );
    }
}

fn format_rational(numerator: &BigInt, denominator: &BigInt, decimals: usize) -> String {
    let scale = BigInt::from(10).pow(decimals as u32);

    let scaled = numerator * &scale;
    use num_integer::Integer;
    let (quotient, remainder) = scaled.div_rem(denominator);

    use num_traits::sign::Signed;
    let rounded = if remainder.abs() * 2 >= denominator.abs() {
        if (numerator.signum() * denominator.signum()).is_negative() {
            quotient - 1
        } else {
            quotient + 1
        }
    } else {
        quotient
    };

    let int_part = &rounded / &scale;
    let frac_part = (&rounded.abs() % &scale).to_string();

    format!("{}.{}", int_part, frac_part)
}
