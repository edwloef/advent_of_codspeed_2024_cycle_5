use ahash::{HashMap, HashMapExt as _, HashSet, HashSetExt as _};
use itertools::Itertools as _;
use std::iter::once;

pub fn part1(input: &str) -> u32 {
    let input = parse(input);

    input
        .keys()
        .map(|node| dfs(&input, node, node, 3, node.as_bytes()[0] == b't'))
        .sum::<u32>()
        / 6
}

fn dfs<'a>(
    graph: &HashMap<&str, HashSet<&'a str>>,
    from: &str,
    cur: &'a str,
    len: u8,
    t_seen: bool,
) -> u32 {
    if len == 1 {
        return (t_seen && graph[cur].contains(&from)).into();
    }

    graph[cur]
        .iter()
        .map(|child| {
            dfs(
                graph,
                from,
                child,
                len - 1,
                t_seen || child.as_bytes()[0] == b't',
            )
        })
        .sum()
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
