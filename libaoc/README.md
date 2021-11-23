# libaoc

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