use ahash::HashMap;
use arrayvec::ArrayVec;
use itertools::Itertools as _;

struct Gate<'a> {
    lhs: &'a str,
    rel: &'a str,
    rhs: &'a str,
    res: &'a str,
}

pub fn part1(input: &str) -> u64 {
    let (values, gates) = input.split_once("\n\n").unwrap();

    let mut values: HashMap<_, _> = values
        .lines()
        .map(|line| (&line[..3], line.as_bytes()[5] == b'1'))
        .collect();

    let mut gates: Vec<_> = gates
        .lines()
        .map(|line| Gate {
            lhs: &line[..3],
            rel: &line[4..line.len() - 11],
            rhs: &line[line.len() - 10..line.len() - 7],
            res: &line[line.len() - 3..],
        })
        .collect();

    while !gates.is_empty() {
        gates.retain(|gate| {
            if !values.contains_key(gate.lhs) || !values.contains_key(gate.rhs) {
                return true;
            }

            values.insert(
                gate.res,
                match gate.rel {
                    "AND" => values[gate.lhs] & values[gate.rhs],
                    "OR" => values[gate.lhs] | values[gate.rhs],
                    "XOR" => values[gate.lhs] ^ values[gate.rhs],
                    _ => unreachable!(),
                },
            );

            false
        });
    }

    let mut res = 0;

    values
        .keys()
        .filter(|key| key.starts_with('z'))
        .sorted_unstable()
        .rev()
        .for_each(|k| {
            res <<= 1;
            res += u64::from(values[k]);
        });

    res
}

pub fn part2(input: &str) -> String {
    let mut gates: Vec<_> = input
        .split_once("\n\n")
        .unwrap()
        .1
        .lines()
        .map(|line| Gate {
            lhs: &line[..3],
            rel: &line[4..line.len() - 11],
            rhs: &line[line.len() - 10..line.len() - 7],
            res: &line[line.len() - 3..],
        })
        .collect();

    let mut swapped = ArrayVec::<&str, 8>::new();
    let mut last_carry = "";

    for i in 0.. {
        let x = &*format!("x{i:02}");
        let y = &*format!("y{i:02}");

        let mut half_adder_1_sum = find(x, y, "XOR", &mut gates);
        if half_adder_1_sum.is_empty() {
            break;
        };

        let mut half_adder_1_carry = find(x, y, "AND", &mut gates);
        let mut full_adder_carry = "";

        if !last_carry.is_empty() {
            let mut half_adder_2_carry = find(last_carry, half_adder_1_sum, "AND", &mut gates);

            if half_adder_2_carry.is_empty() {
                std::mem::swap(&mut half_adder_1_sum, &mut half_adder_1_carry);
                swapped.extend([half_adder_1_sum, half_adder_1_carry]);

                half_adder_2_carry = find(last_carry, half_adder_1_sum, "AND", &mut gates);
            }

            let mut half_adder_2_sum = find(last_carry, half_adder_1_sum, "XOR", &mut gates);

            if half_adder_1_sum.starts_with('z') {
                std::mem::swap(&mut half_adder_1_sum, &mut half_adder_2_sum);
                swapped.extend([half_adder_1_sum, half_adder_2_sum]);
            }

            if half_adder_1_carry.starts_with('z') {
                std::mem::swap(&mut half_adder_1_carry, &mut half_adder_2_sum);
                swapped.extend([half_adder_1_carry, half_adder_2_sum]);
            }

            if half_adder_2_carry.starts_with('z') {
                std::mem::swap(&mut half_adder_2_carry, &mut half_adder_2_sum);
                swapped.extend([half_adder_2_carry, half_adder_2_sum]);
            }

            full_adder_carry = find(half_adder_2_carry, half_adder_1_carry, "OR", &mut gates);

            if full_adder_carry.starts_with('z') && full_adder_carry != "z45" {
                std::mem::swap(&mut full_adder_carry, &mut half_adder_2_sum);
                swapped.extend([full_adder_carry, half_adder_2_sum]);
            }
        }

        last_carry = if last_carry.is_empty() {
            half_adder_1_carry
        } else {
            full_adder_carry
        };
    }

    swapped.iter().sorted_unstable().join(",")
}

fn find<'a>(lhs: &str, rhs: &str, rel: &str, gates: &mut Vec<Gate<'a>>) -> &'a str {
    let Some(position) = gates.iter().position(|gate| {
        gate.rel == rel
            && ((gate.lhs == lhs && gate.rhs == rhs) || (gate.lhs == rhs && gate.rhs == lhs))
    }) else {
        return "";
    };
    gates.remove(position).res
}
