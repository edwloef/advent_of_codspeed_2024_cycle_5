use ahash::{HashMap, HashMapExt as _, HashSet, HashSetExt as _};
use itertools::Itertools as _;
use std::iter::once;

pub fn part1(input: &str) -> u32 {
    let input = parse(input);

    let mut count = 0;

    for node1 in input.keys() {
        let t1 = node1.starts_with('t');
        for node2 in &input[node1] {
            let t2 = node2.starts_with('t');
            for node3 in &input[node2] {
                let t3 = node3.starts_with('t');
                if (t1 || t2 || t3) && input[node3].contains(node1) {
                    count += 1;
                }
            }
        }
    }

    count / 6
}

pub fn part2(input: &str) -> String {
    let input = parse(input);

    bron_kerbosch(
        &input,
        &mut HashSet::new(),
        &mut input.keys().copied().collect(),
        &mut HashSet::new(),
    )
    .into_iter()
    .sorted_unstable()
    .join(",")
}

fn bron_kerbosch<'a>(
    graph: &HashMap<&str, HashSet<&'a str>>,
    r: &mut HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
) -> HashSet<&'a str> {
    if p.is_empty() && x.is_empty() {
        return r.clone();
    }

    let u = p.union(x).next().unwrap();

    let mut p_intersection = HashSet::new();
    let mut x_intersection = HashSet::new();

    p.clone()
        .difference(&graph[u])
        .map(|&v| {
            r.insert(v);

            p_intersection.clear();
            x_intersection.clear();

            p_intersection.extend(p.intersection(&graph[v]).copied());
            x_intersection.extend(x.intersection(&graph[v]).copied());

            let out = bron_kerbosch(graph, r, &mut p_intersection, &mut x_intersection);

            r.remove(v);
            p.remove(v);
            x.insert(v);

            out
        })
        .max_by(|lhs, rhs| lhs.len().cmp(&rhs.len()))
        .unwrap_or_else(HashSet::new)
}

fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut out = HashMap::new();

    input
        .lines()
        .map(|line| (&line[..2], &line[3..]))
        .for_each(|(from, to)| {
            out.entry(from)
                .and_modify(|x: &mut HashSet<_>| {
                    x.insert(to);
                })
                .or_insert_with(|| once(to).collect());

            out.entry(to)
                .and_modify(|x: &mut HashSet<_>| {
                    x.insert(from);
                })
                .or_insert_with(|| once(from).collect());
        });

    out
}
