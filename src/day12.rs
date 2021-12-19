use std::collections::HashMap;
use std::hash::Hash;

use dashmap::DashMap;
use itertools::Itertools;
use tap::{Pipe, Tap};

/// https://adventofcode.com/2021/day/12

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Cave {
    identifier: String,
}

fn frequency<T: Clone + Eq + Hash>(input: Vec<T>) -> HashMap<T, usize> {
    fn bind<T: Clone + Eq + Hash>(
        input: Vec<T>,
        output: &mut HashMap<T, usize>,
    ) -> HashMap<T, usize> {
        input
            .into_iter()
            .for_each(|x| {
                output
                    .entry(x)
                    .and_modify(|y| *y += 1)
                    .or_insert(1)
                    .pipe(|_| ())
            })
            .pipe(|_| std::mem::take(output))
    }
    bind(input, &mut HashMap::new())
}

impl Cave {
    fn big(&self) -> bool {
        self.identifier.chars().nth(0).unwrap().is_uppercase()
    }
}

#[aoc_generator(day12)]
pub fn generator(input: &str) -> DashMap<Cave, Vec<Cave>> {
    fn bind(input: &str, bins: DashMap<Cave, Vec<Cave>>) -> DashMap<Cave, Vec<Cave>> {
        input
            .lines()
            .map(|x| {
                x.split("-")
                    .map(|y| Cave {
                        identifier: y.to_string(),
                    })
                    .collect_tuple::<(Cave, Cave)>()
                    .unwrap()
            })
            .for_each(|(a, b)| {
                bins.entry(a.clone())
                    .and_modify(|x| x.push(b.clone()))
                    .or_insert(vec![b.clone()])
                    .pipe(|_| ())
                    .pipe(|_| {
                        bins.entry(b.clone())
                            .and_modify(|x| x.push(a.clone()))
                            .or_insert(vec![a.clone()])
                            .pipe(|_| ())
                    })
            })
            .pipe(|_| bins)
    }
    bind(input, DashMap::new())
}

fn all_simple_paths(
    graph: &DashMap<Cave, Vec<Cave>>,
    small_visit: usize,
    num_over: usize,
) -> Vec<Vec<Cave>> {
    fn recurse(
        graph: &DashMap<Cave, Vec<Cave>>,
        node: Cave,
        target: Cave,
        paths: &mut Vec<Vec<Cave>>,
        mut path: Vec<Cave>,
        visited: &DashMap<Cave, usize>,
        small_visit: usize,
        num_over: usize,
    ) {
        visited
            .entry(node.clone())
            .and_modify(|x| *x += 1)
            .or_insert(1)
            .pipe(|_| ())
            .pipe(|_| {
                path.push(node.clone()).pipe(|()| {
                    if node == target {
                        paths.push(path.clone())
                    } else {
                        if let Some(x) = graph.get(&node) {
                            x.iter()
                                .filter(|x| {
                                    (x.big()
                                        || (*visited.entry(x.clone().clone()).or_insert(0).value()
                                            < 1)
                                        || (*visited.get(x.clone()).unwrap().value() < small_visit
                                            && frequency(
                                                path.iter().filter(|z| !z.big()).collect(),
                                            )
                                            .iter()
                                            .map(|(_, b)| b)
                                            .filter(|b| **b > 1)
                                            .count()
                                                <= num_over))
                                        && x.identifier != "start"
                                })
                                .for_each(|next| {
                                    recurse(
                                        graph,
                                        next.clone(),
                                        target.clone(),
                                        paths,
                                        path.clone(),
                                        visited,
                                        small_visit,
                                        num_over,
                                    )
                                })
                        }
                    }
                })
            })
            .pipe(|_| {
                visited
                    .entry(node.clone())
                    .and_modify(|x| *x -= 1)
                    .pipe(|_| ())
            })
    }
    fn bind(
        graph: &DashMap<Cave, Vec<Cave>>,
        paths: &mut Vec<Vec<Cave>>,
        small_visit: usize,
        num_over: usize,
    ) -> Vec<Vec<Cave>> {
        recurse(
            graph,
            Cave {
                identifier: "start".to_string(),
            },
            Cave {
                identifier: "end".to_string(),
            },
            paths,
            vec![],
            &DashMap::new()
                .tap_mut(|x| {
                    x.insert(
                        Cave {
                            identifier: "start".to_string(),
                        },
                        small_visit - 1,
                    )
                    .pipe(|_| ())
                })
                .tap_mut(|x| {
                    x.insert(
                        Cave {
                            identifier: "end".to_string(),
                        },
                        small_visit - 1,
                    )
                    .pipe(|_| ())
                }),
            small_visit,
            num_over,
        )
        .pipe(|()| paths.clone())
    }
    bind(graph, &mut vec![], small_visit, num_over)
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &DashMap<Cave, Vec<Cave>>) -> usize {
    all_simple_paths(input, 1, 0).len()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &DashMap<Cave, Vec<Cave>>) -> usize {
    all_simple_paths(input, 2, 1).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part1() {
        let provided = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

        assert_eq!(19, solve_part1(&generator(provided)));
        assert_eq!(
            3576,
            solve_part1(&generator(
                include_str!("../input/2021/day12.txt").trim_end()
            ))
        );
    }

    #[test]
    fn it_works_part2() {
        let provided = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

        assert_eq!(36, solve_part2(&generator(provided)));
        assert_eq!(
            103,
            solve_part2(&generator(
                "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"
            ))
        );
        assert_eq!(
            3509,
            solve_part2(&generator(
                "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"
            ))
        );
    }
}
