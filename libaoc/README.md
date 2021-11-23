# libaoc

## Usage

To use this library, the environment variable `CURL_AOC_SESSION` must be set to your session id. This can be done by using a `.env` file and the rust crate [`dotenv`](https://docs.rs/dotenv/0.15.0/dotenv/).

## Example

```rust
use libaoc::{Aoc, Day, DayNumber};

fn main() {
    let mut aoc = Aoc<2021>::new();

    aoc.add_day(Day::new(DayNumber::One, |input| {
        println!("{}", input);
    }));

    aoc.run_days_all();
}
```