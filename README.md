# Advent of Code 2023

Not much changing here, [Advent of Code](https://adventofcode.com) 2024!
Sticking to Rust this year as well.

## Input

As per [the official FAQ][faq] and a lot of [discussions on Reddit][reddit] I'm
not committing any input data this year. So to run this code you'll have to
download the input first.

## Test

```sh
# Test all days so far
› cargo test
```

## Execute

```sh
# Run a specific day
› cargo run <day: i32>
```

## Setup new day

This is very specific for this repository, but the script [`new-day`](./new-day)
will check the `solutions` module to see how many days have been solved and then
scaffold an empty day and update the match arms to include this.

If the environment variable `AOC_TOKEN` is set to a valid session cookie value
the input for that day will be downloaded as well.

[faq]: https://adventofcode.com/about#faq_copying
[reddit]: https://www.reddit.com/r/adventofcode/wiki/troubleshooting/no_asking_for_inputs
