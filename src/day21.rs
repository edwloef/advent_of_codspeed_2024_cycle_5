use ahash::{HashMap, HashMapExt as _};
use arrayvec::ArrayVec;
use itertools::Itertools as _;
use std::iter::{once, repeat_n};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Keypad(i8, i8);

impl Keypad {
    const A: Self = Self(0, 0);

    const ZERO: Self = Self(1, 0);
    const ONE: Self = Self(2, 1);
    const TWO: Self = Self(1, 1);
    const THREE: Self = Self(0, 1);
    const FOUR: Self = Self(2, 2);
    const FIVE: Self = Self(1, 2);
    const SIX: Self = Self(0, 2);
    const SEVEN: Self = Self(2, 3);
    const EIGHT: Self = Self(1, 3);
    const NINE: Self = Self(0, 3);

    const ARROW_UP: Self = Self(1, 0);
    const ARROW_RIGHT: Self = Self(0, -1);
    const ARROW_DOWN: Self = Self(1, -1);
    const ARROW_LEFT: Self = Self(2, -1);

    fn from_byte(c: u8) -> Self {
        match c {
            b'A' => Self::A,
            b'0' => Self::ZERO,
            b'1' => Self::ONE,
            b'2' => Self::TWO,
            b'3' => Self::THREE,
            b'4' => Self::FOUR,
            b'5' => Self::FIVE,
            b'6' => Self::SIX,
            b'7' => Self::SEVEN,
            b'8' => Self::EIGHT,
            b'9' => Self::NINE,
            _ => unreachable!(),
        }
    }
}

pub fn part1(input: &str) -> u64 {
    solve(input, 3)
}

pub fn part2(input: &str) -> u64 {
    solve(input, 26)
}

fn solve(input: &str, depth: u8) -> u64 {
    let mut memo = HashMap::new();

    input
        .lines()
        .map(|line| {
            let mut numeric = 0;
            line.bytes().filter(|&c| c != b'A').for_each(|c| {
                numeric *= 10;
                numeric += u64::from(c - b'0');
            });

            let length = once(Keypad::A)
                .chain(line.bytes().map(Keypad::from_byte))
                .tuple_windows()
                .map(|(last, cur)| recursive(last, cur, depth, &mut memo))
                .sum::<u64>();

            numeric * length
        })
        .sum()
}

fn recursive(
    last: Keypad,
    cur: Keypad,
    depth: u8,
    memo: &mut HashMap<(Keypad, Keypad, u8), u64>,
) -> u64 {
    if depth == 0 {
        return 1;
    }

    if let Some(result) = memo.get(&(last, cur, depth)) {
        return *result;
    }

    let mut expand = ArrayVec::<Keypad, 6>::new();
    expand.push(Keypad::A);

    if last.0 > cur.0 {
        expand.extend(repeat_n(Keypad::ARROW_RIGHT, last.0.abs_diff(cur.0).into()));
    }

    if last.1 < cur.1 {
        expand.extend(repeat_n(Keypad::ARROW_UP, last.1.abs_diff(cur.1).into()));
    }

    if last.1 > cur.1 {
        expand.extend(repeat_n(Keypad::ARROW_DOWN, last.1.abs_diff(cur.1).into()));
    }

    if last.0 < cur.0 {
        expand.extend(repeat_n(Keypad::ARROW_LEFT, last.0.abs_diff(cur.0).into()));
    }

    expand.push(Keypad::A);

    let mut count = expand
        .iter()
        .tuple_windows()
        .map(|(&last, &cur)| recursive(last, cur, depth - 1, memo))
        .sum::<u64>();

    if !(last.1 == 0 && cur.0 == 2 || cur.1 == 0 && last.0 == 2) {
        count = count.min(
            expand
                .iter()
                .rev()
                .tuple_windows()
                .map(|(&last, &cur)| recursive(last, cur, depth - 1, memo))
                .sum(),
        );
    }

    memo.insert((last, cur, depth), count);

    count
}
