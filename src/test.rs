use super::*;

/*
       0
    3     1
       2

Starting at face 0, we can work out all the directions we can reach in 4 moves
we need to get to face 2 last
W - win
X - Loss
T - tie

LLLL X      RLLL W
LLLR X      RLLR W
LLRL X      RLRL T
LLRR X      RLRR X
LRLL X      RRLL X
LRLR T      RRLR X
LRRL W      RRRL X
LRRR W      RRRR X

total:
Wins: 4
Losses: 10
Ties: 2

*/
#[test]
fn test_solve_iterative() {
    let Results { wins, losses, ties } = solve_iterative(4, 4);
    assert_eq!(wins.to_u64_digits()[0], 4);
    assert_eq!(losses.to_u64_digits()[0], 10);
    assert_eq!(ties.to_u64_digits()[0], 2);
}

#[test]
fn tttt() {
    panic!()
}
