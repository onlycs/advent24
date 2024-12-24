use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use itertools::Itertools;
use libadvent::{FuncParser, Parser, Seperated};

pub struct Graph<N> {
    edges: HashMap<N, HashSet<N>>,
}

impl<N: Hash + Eq + Copy + Debug + Ord> Graph<N> {
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    fn add_connection(&mut self, a: N, b: N) {
        self.edges.entry(a).or_default().insert(b);
        self.edges.entry(b).or_default().insert(a);
    }

    fn collect(&self) -> HashSet<[N; 3]> {
        let mut triplets = HashSet::new();

        for pc in self.edges.keys().copied() {
            // either edges or edges_rev must contain the node
            let others = &self.edges[&pc];

            for (a, b) in others.iter().tuple_combinations() {
                // we know that a's connections contain node and b's connection contains node
                // check to make sure that a and b are connected
                let a_conns = &self.edges[a];

                if !a_conns.contains(b) {
                    continue;
                }

                let mut arr = [pc, *a, *b];
                arr.sort();

                triplets.insert(arr);
            }
        }

        triplets
    }

    // thank you wikipedia
    fn bron_kerbosch(
        &self,
        r: &mut HashSet<N>,
        mut p: HashSet<N>,
        mut x: HashSet<N>,
        cliques: &mut Vec<HashSet<N>>,
    ) {
        if p.is_empty() {
            if x.is_empty() {
                cliques.push(r.clone());
            }

            return;
        }

        while let Some(vert) = p.iter().copied().next() {
            let n = &self.edges[&vert];

            let p2 = p.intersection(n).copied().collect();
            let x2 = x.intersection(n).copied().collect();

            r.insert(vert);
            self.bron_kerbosch(r, p2, x2, cliques);
            r.remove(&vert);

            p.remove(&vert);
            x.insert(vert);
        }
    }
}

pub struct InputParser;

impl Parser for InputParser {
    type Output = Graph<[char; 2]>;

    fn parse(&mut self, s: &str) -> Self::Output {
        let mut graph = Graph::new();

        Seperated::newline(FuncParser::new(|line| {
            let chars = line.chars().collect_vec();
            let node1 = &chars[0..2];
            let node2 = &chars[3..];

            let node1 = <[char; 2]>::try_from(node1).unwrap();
            let node2 = <[char; 2]>::try_from(node2).unwrap();

            graph.add_connection(node1, node2);
        }))
        .parse(s);

        graph
    }
}

problem_parser!(InputParser => Graph<[char; 2]>);

pub fn level1(g: Graph<[char; 2]>) -> usize {
    g.collect()
        .into_iter()
        .filter(|arr| arr.iter().any(|it| it[0] == 't'))
        .count()
}

pub fn level2(g: Graph<[char; 2]>) -> String {
    let mut cliques = vec![];

    g.bron_kerbosch(
        &mut HashSet::new(),
        g.edges.keys().copied().collect(),
        HashSet::new(),
        &mut cliques,
    );

    let mut clique = cliques
        .into_iter()
        .max_by_key(|clique| clique.len())
        .unwrap()
        .into_iter()
        .collect_vec();

    clique.sort();
    clique.into_iter().map(|it| it.iter().join("")).join(",")
}
