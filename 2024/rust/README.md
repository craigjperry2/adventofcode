# AoC Rust 2024

Remind myself of stuff i learned before:

* Integer types silently wrap-around on overflow in rust release mode. Run
  in debug mode when developing to check for and panic in this case
* A couple of things are missing in the rust standard library that might
  be useful in AoC:
  * To use regular expressions in Rust, you need to `cargo add regex` - it's
    not part of the standard library but there is create with support for
    basic regexp usage - it's not full PCRE though (e.g. no backtracking)
  * `itertools` crate adds some time saving conveniences to iterators
  * To use random numbers, you should `cargo add rand` for a solid set of
    functions to generate different types of random number safely
  * Date / time handling in the standard library is a bit anemic, you
    can `cargo add chrono` for a nicer API surface
* There's some common libs in the rust landscape that end up being added
  to my side-projects but maybe not needed for AoC, i'll journal what i
  remeber in case i do need one of these:
  * `anyhow` for more ergonomic error handling
  * `tracing` for logging and it's compatible with open telemetry
  * `once_cell` for singletons
  * `bitflags` for verbose but correct handling of bit flags
  * The `dotenv` crate is unmaintained and has a sec advisory - use dotenvy
    instead. Or just don't use dotenv, it's not really needed here

