#![allow(dead_code)]
#![allow(unused_variables)]

use std::time::Instant;

fn main() {
    let faces: i8 = 12;
    let max_iterations: i8 = 20;

    assert!(faces % 2 == 0);

    recursive_solution::solve_recursive(faces, max_iterations);
    solve_iterative(faces, max_iterations);
}

fn solve_iterative(faces: i8, max_iterations: i8) {
    let target: i8 = faces / 2;
    let start = Instant::now();

    let mut win: u128 = 0;
    let mut loss: u128 = 0;
    let mut tie: u128 = 0;

    println!(
        "Calculating outcomes for target {}, faces {}, iterations {}",
        target, faces, max_iterations
    );

    let mut seen_none = vec![0u128; (faces - 1) as usize];
    let mut seen_left = vec![0u128; (faces - 1) as usize];
    let mut seen_right = vec![0u128; (faces - 1) as usize];
    let shift = (faces - 2) / 2;

    seen_none[shift as usize] = 1;

    for current_iteration in 0..max_iterations {
        let mut next_seen_none = vec![0u128; (faces - 1) as usize];
        let mut next_seen_left = vec![0u128; (faces - 1) as usize];
        let mut next_seen_right = vec![0u128; (faces - 1) as usize];

        for (i, n) in seen_none.iter().enumerate() {
            if *n == 0 {
                continue;
            }
            if i == 1 {
                next_seen_none[i + 1] += n;
                next_seen_left[i - 1] += n;
                continue;
            }
            if i == (faces - 3) as usize {
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
            if i == 0 {
                loss += n << (max_iterations - current_iteration - 1) as u128;
                next_seen_left[i + 1] += n;
                continue;
            }
            if i == (faces - 2) as usize {
                win += (2 * n) << (max_iterations - current_iteration - 1) as u128;
                continue;
            }
            next_seen_left[i - 1] += n;
            next_seen_left[i + 1] += n;
        }
        for (i, n) in seen_right.iter().enumerate() {
            if *n == 0 {
                continue;
            }
            if i == 0 {
                win += (2 * n) << (max_iterations - current_iteration - 1) as u128;
                continue;
            }
            if i == (faces - 2) as usize {
                loss += n << (max_iterations - current_iteration - 1) as u128;
                next_seen_right[i - 1] += n;
                continue;
            }
            next_seen_right[i - 1] += n;
            next_seen_right[i + 1] += n;
        }

        seen_none = next_seen_none;
        seen_left = next_seen_left;
        seen_right = next_seen_right;
    }

    win += seen_left[faces as usize - 2];
    seen_left[faces as usize - 2] = 0;
    loss += seen_right[0];
    seen_right[0] = 0;

    tie += seen_none.iter().sum::<u128>();
    tie += seen_left.iter().sum::<u128>();
    tie += seen_right.iter().sum::<u128>();

    println!(
        "Wins: {}, Losses: {}, Ties: {}, Total: {}",
        win,
        loss,
        tie,
        win + loss + tie
    );
    println!(
        "{:.10} <= p <= {:.10}",
        (win as f64 / (win + loss + tie) as f64),
        ((win + tie) as f64 / (win + loss + tie) as f64)
    );
    println!("Guess p = {:.10}", 1 as f64 / (faces - 1) as f64);

    let elapsed = start.elapsed();
    println!("Finished in {:?}", elapsed);
}

mod recursive_solution;
