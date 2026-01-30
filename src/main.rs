#![allow(dead_code)]
#![allow(unused_variables)]

use std::time::Instant;

fn main() {
    let faces: i8 = 6;
    let max_iterations: i8 = 35;

    solve_recursive(faces, max_iterations);
}

// fn solve_iterative(faces: i8, max_iterations: i8) {
//     let target: i8 = faces / 2;
//     let start = Instant::now();

//     println!(
//         "Calculating outcomes for target {}, faces {}, iterations {}",
//         target, faces, max_iterations
//     );

//     let seen_none = vec![0u128; faces as usize];
//     let seen_left = vec![0u128; faces as usize];
//     let seen_right = vec![0u128; faces as usize];

//     for _ in 0..max_iterations {}

//     let elapsed = start.elapsed();
//     println!("Finished in {:?}", elapsed);
// }

fn solve_recursive(faces: i8, max_iterations: i8) {
    let mut win: u128 = 0;
    let mut loss: u128 = 0;
    let mut tie: u128 = 0;
    let target: i8 = faces / 2;
    let start = Instant::now();

    println!(
        "Calculating outcomes for target {}, faces {}, iterations {}",
        target, faces, max_iterations
    );

    solve(
        0,
        0,
        false,
        false,
        &mut win,
        &mut loss,
        &mut tie,
        target,
        faces,
        max_iterations,
    );

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

fn solve(
    i: i8,
    n: i8,
    mut l: bool,
    mut r: bool,
    win: &mut u128,
    loss: &mut u128,
    tie: &mut u128,
    target: i8,
    faces: i8,
    max_iterations: i8,
) {
    if n == target {
        *loss += 1 << (max_iterations - i) as u128;
        return;
    }
    if l && n == target + 1 {
        *win += 1 << (max_iterations - i) as u128;
        return;
    }
    if r && n == target - 1 {
        *win += 1 << (max_iterations - i) as u128;
        return;
    }
    if n == target - 1 {
        l = true;
    }
    if n == target + 1 {
        r = true;
    }
    if i == max_iterations {
        *tie += 1;
        return;
    }

    solve(
        i + 1,
        (n + faces - 1) % faces,
        l,
        r,
        win,
        loss,
        tie,
        target,
        faces,
        max_iterations,
    );
    solve(
        i + 1,
        (n + 1) % faces,
        l,
        r,
        win,
        loss,
        tie,
        target,
        faces,
        max_iterations,
    );
}
