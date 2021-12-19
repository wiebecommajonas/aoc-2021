use std::ops::Add;

use libaoc::{Day, DayNumber};

#[derive(Clone, Debug, PartialEq)]
struct SnailfishNumber {
    value: Option<usize>,
    left: Option<Box<SnailfishNumber>>,
    right: Option<Box<SnailfishNumber>>,
}

impl SnailfishNumber {
    fn new_leaf(value: usize) -> Self {
        Self {
            value: Some(value),
            left: None,
            right: None,
        }
    }

    fn new_empty() -> Self {
        Self {
            value: None,
            left: None,
            right: None,
        }
    }

    fn is_empty(&self) -> bool {
        *self == Self::new_empty()
    }

    fn magnitude(&self) -> usize {
        if self.value.is_some() {
            self.value.unwrap()
        } else {
            3 * self.left.as_ref().unwrap().magnitude()
                + 2 * self.right.as_ref().unwrap().magnitude()
        }
    }

    fn reduce_explode(&mut self, depth: usize) -> (Option<usize>, Option<usize>, bool) {
        if depth == 4
            && self.left.is_some()
            && self.right.is_some()
            && self.left.as_ref().unwrap().value.is_some()
            && self.right.as_ref().unwrap().value.is_some()
        {
            let l = self.left.as_ref().unwrap().value.unwrap();
            let r = self.right.as_ref().unwrap().value.unwrap();
            self.left = None;
            self.right = None;
            self.value = Some(0);
            return (Some(l), Some(r), true);
        }

        if self.left.is_some() {
            if let (l, r, true) = self.left.as_mut().unwrap().reduce_explode(depth + 1) {
                if let Some(r_val) = r {
                    let mut current = self.right.as_mut().unwrap();
                    while current.value.is_none() {
                        current = current.left.as_mut().unwrap();
                    }
                    current.value = Some(current.value.unwrap() + r_val);
                }
                return (l, None, true);
            }
        }

        if self.right.is_some() {
            if let (l, r, true) = self.right.as_mut().unwrap().reduce_explode(depth + 1) {
                if let Some(l_val) = l {
                    let mut current = self.left.as_mut().unwrap();
                    while current.value.is_none() {
                        current = current.right.as_mut().unwrap();
                    }
                    current.value = Some(current.value.unwrap() + l_val);
                }
                return (None, r, true);
            }
        }

        (None, None, false)
    }

    fn reduce_split(&mut self) -> bool {
        if let Some(n) = self.value {
            if n >= 10 {
                let l = n / 2;
                let r;
                if n % 2 == 0 {
                    r = n / 2;
                } else {
                    r = n / 2 + 1;
                }
                self.value = None;
                self.left = Some(Box::new(Self::new_leaf(l)));
                self.right = Some(Box::new(Self::new_leaf(r)));
                return true;
            }
        }

        if self.left.is_some() && self.left.as_mut().unwrap().reduce_split() {
            true
        } else {
            self.right.is_some() && self.right.as_mut().unwrap().reduce_split()
        }
    }

    fn reduce(&mut self) -> bool {
        if let (_, _, true) = self.reduce_explode(0) {
            true
        } else {
            self.reduce_split()
        }
    }

    fn from_str(input: &str) -> Self {
        if &input[0..1] != "[" {
            Self::new_leaf(input.parse::<usize>().unwrap())
        } else {
            let mut comma_pos = 0;
            let mut level = 0;
            for ch in input.chars() {
                match ch {
                    '[' => level += 1,
                    ']' => level -= 1,
                    ',' if level == 1 => break,
                    _ => (),
                }
                comma_pos += 1;
            }
            Self {
                value: None,
                left: Some(Box::new(Self::from_str(
                    input[1..].split_at(comma_pos - 1).0,
                ))),
                right: Some(Box::new(Self::from_str(
                    input[..input.len() - 1].split_at(comma_pos + 1).1,
                ))),
            }
        }
    }
}

impl Add<SnailfishNumber> for SnailfishNumber {
    type Output = SnailfishNumber;
    fn add(self, rhs: SnailfishNumber) -> Self::Output {
        let mut result;
        if self.is_empty() {
            result = rhs
        } else if rhs.is_empty() {
            result = self
        } else {
            result = Self {
                value: None,
                left: Some(Box::new(self)),
                right: Some(Box::new(rhs)),
            }
        }
        while result.reduce() {}
        result
    }
}

pub fn eighteen() -> Day<2021> {
    Day::new(
        DayNumber::Eighteen,
        |input| {
            Box::new(
                input
                    .lines()
                    .map(|line| SnailfishNumber::from_str(line))
                    .fold(SnailfishNumber::new_empty(), |acc, next| acc + next)
                    .magnitude(),
            )
        },
        |input| {
            let numbers: Vec<SnailfishNumber> = input
                .lines()
                .map(|line| SnailfishNumber::from_str(line))
                .collect();

            let mut max_magnitude = 0;
            for x in &numbers {
                for y in &numbers {
                    let magnitude = (x.clone() + y.clone()).magnitude();
                    if magnitude > max_magnitude {
                        max_magnitude = magnitude;
                    }
                }
            }
            Box::new(max_magnitude)
        },
    )
}

#[cfg(test)]
mod tests {
    use super::SnailfishNumber;

    #[test]
    fn parse() {
        let input = "[[[5,[2,8]],4],[5,[[9,9],0]]]";
        SnailfishNumber::from_str(input);
    }

    #[test]
    fn reduce() {
        let mut input = SnailfishNumber::from_str("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]");
        let result = SnailfishNumber::from_str("[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
        input.reduce();
        assert_eq!(input, result);
    }

    #[test]
    fn sum() {
        let input = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]";
        let sum = input
            .lines()
            .map(|line| SnailfishNumber::from_str(line))
            .fold(SnailfishNumber::new_empty(), |acc, next| acc + next);
        assert_eq!(
            sum,
            SnailfishNumber::from_str("[[[[5,0],[7,4]],[5,5]],[6,6]]")
        )
    }

    #[test]
    fn magnitude() {
        assert_eq!(
            SnailfishNumber::from_str("[[1,2],[[3,4],5]]").magnitude(),
            143
        );
        assert_eq!(
            SnailfishNumber::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(),
            1384
        );
        assert_eq!(
            SnailfishNumber::from_str("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(),
            445
        );
        assert_eq!(
            SnailfishNumber::from_str("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(),
            791
        );
        assert_eq!(
            SnailfishNumber::from_str("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(),
            1137
        );
        assert_eq!(
            SnailfishNumber::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
                .magnitude(),
            3488
        );
    }
}
