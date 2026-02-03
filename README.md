# Clock Puzzle Solver

This project is based on the 'ladybug clock puzzle' posted by 3blue1brown (https://www.youtube.com/shorts/t3jZ2xGOvYg)

My first attempt was a recursive solution and it was able to go 40 iterations in about 30 seconds on my laptop.
This solution does 10,000 iterations in less than 1 second.

This small project let me explore bigint which let me store arbirarily large unsigned integers without thinking about the implementation.

# Building / Running

In order to run the project you need to have rust installed, install rust/cargo through https://rustup.rs/.

## Running the project

The easiest way to run the project is directly running it with cargo.

Running the solver for a 12 face clock for 100 iterations would look like this:

`cargo run --release -- 12 100`

By default you will see the range of probabilities with 10 decimal places of accuracy, but you can also change the amount of decimals with a third argument:

`cargo run --release -- 12 100 25`

If you would like to run the test case that I have worked out by hand you can do that with:

`cargo test`
