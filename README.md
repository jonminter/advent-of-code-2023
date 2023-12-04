# advent-of-code-2023

Over engineered solutions to Advent of Code 2023

## How is this over-engineered?

- No panics except where I think it is impossible for a panic statement to be hit and should have asserts that verify my assumptions (ex: we checked that a char is a digit to calling `to_digit` should never panic so using `unwrap` is ok here). Otherwise, must return a `Result` and the error should be handled properly
- Where reasonable should use an efficient algorithm. Reduce time & memory usage complexity.
- Use the type system as much as possible to prevent possibility of logic errors.
- Write code like this is production code, should be readable/understandable
