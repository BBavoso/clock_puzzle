#[allow(dead_code)]
pub(crate) fn solve_recursive(faces: i8, max_iterations: i8) -> crate::Results {
    let mut win: u128 = 0;
    let mut loss: u128 = 0;
    let mut tie: u128 = 0;
    let target: i8 = faces / 2;

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

    crate::Results {
        wins: win,
        losses: loss,
        ties: tie,
    }
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
