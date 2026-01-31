use std::time::Instant;

#[cfg(test)]
mod test;

fn main() {
    let faces: i8 = 10;
    let max_iterations: i8 = 127;

    assert_eq!(faces % 2, 0, "Faces must be even");

    let start = Instant::now();

    println!(
        "Solving for total outcomes of a clock puzzle with {} faces, doing {} iterations",
        faces, max_iterations
    );

    let results = solve_iterative(faces, max_iterations);

    results.print_results(faces, max_iterations);

    let elapsed = start.elapsed();
    println!("Finished in {:?}", elapsed);
}

struct Results {
    wins: u128,
    losses: u128,
    ties: u128,
}

impl Results {
    fn print_results(&self, faces: i8, max_iterations: i8) {
        println!(
            "Results for faces: {}, iterations: {}",
            faces, max_iterations
        );
        println!("Wins: {}", self.wins);
        println!("Losses: {}", self.losses);
        println!("Ties: {}", self.ties);
        println!("Total: {}", self.wins + self.losses + self.ties);
        println!("Guess p = {:.10}", 1_f64 / (faces - 1) as f64);
        println!(
            "{:.10} <= p <= {:.10}",
            (self.wins as f64 / (self.wins + self.losses + self.ties) as f64),
            ((self.wins + self.ties) as f64 / (self.wins + self.losses + self.ties) as f64)
        );
    }
}

fn solve_iterative(faces: i8, max_iterations: i8) -> Results {
    let mut wins: u128 = 0;
    let mut losses: u128 = 0;

    // arrays to hold the number of ways to reach each position
    // the seen left and right arrays indicate whether we've seen
    // the face immediately adjacent to the target face
    // note that none seen could be 2 faces smaller and left and right
    // could be 1 face smaller but this size alligns them much easier
    let seen_size = (faces - 1) as usize;
    let mut seen_none = vec![0u128; seen_size];
    let mut seen_left = vec![0u128; seen_size];
    let mut seen_right = vec![0u128; seen_size];

    // the array is shifted such that the middle index
    // is the starting face (12 o'clock)
    let shift = (faces - 2) / 2;

    seen_none[shift as usize] = 1;

    for current_iteration in 0..max_iterations {
        let mut next_seen_none = vec![0u128; seen_size];
        let mut next_seen_left = vec![0u128; seen_size];
        let mut next_seen_right = vec![0u128; seen_size];

        // earlier rounds have more weight because
        // the possible outcomes double each iteration
        let weight = (max_iterations - current_iteration - 1) as u128;

        for (i, n) in seen_none.iter().enumerate() {
            if *n == 0 {
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
            if *n == 0 {
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
            if *n == 0 {
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

    let ties = seen_none.iter().sum::<u128>()
        + seen_left.iter().sum::<u128>()
        + seen_right.iter().sum::<u128>();

    Results { wins, losses, ties }
}
