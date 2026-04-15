# CSES Solutions

Reference solutions for CSES problems written in Rust with this repository's
competitive programming template.

## Running the code
The repository for running my code is [rust-cp-template](https://github.com/jeffilluminati/rust-cp-template).
Just replace any one of the `src/bin/{a-g}.rs` with the input code, and run it using `cargo r --bin {a-g}`. 

The solutions rely on the shared `cp` crate and its macros such as
`cp::prepare!()`, `sc!`, `pp!`, and `cp::main!()`. Because of that, they are
meant to be read or copied into one of the actual binaries under `src/bin/`
when you want to run or submit them.
