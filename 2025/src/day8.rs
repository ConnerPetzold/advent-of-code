use aoc_runner_derive::{aoc, aoc_generator};
use glam::{I64Vec3, i64vec3};
use itertools::Itertools;
use nom::{
    Parser,
    bytes::complete::tag,
    character::complete::{i64, newline},
    multi::{separated_list0, separated_list1},
};
use petgraph::{
    algo::{connected_components, kosaraju_scc},
    graph::UnGraph,
};

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<I64Vec3> {
    separated_list0(newline, parse_vec3).parse(input).unwrap().1
}

fn parse_vec3(input: &str) -> nom::IResult<&str, I64Vec3> {
    separated_list1(tag(","), i64)
        .parse(input)
        .map(|(s, vec)| (s, i64vec3(vec[0], vec[1], vec[2])))
}

#[aoc(day8, part1)]
pub fn solve_part1(points: &[I64Vec3]) -> usize {
    let (graph, _) = circuit_graph(points, 1000);
    let circuits = circuits(&graph);
    circuits.iter().map(|c| c.len()).take(3).product()
}

#[aoc(day8, part2)]
pub fn solve_part2(points: &[I64Vec3]) -> i64 {
    let (_, distance) = circuit_graph(points, 499500);
    distance
}

fn circuit_graph(points: &[I64Vec3], min_edges: usize) -> (UnGraph<I64Vec3, ()>, i64) {
    let mut graph = UnGraph::default();

    let node_indices: Vec<_> = points.iter().map(|&p| graph.add_node(p)).collect();

    let mut longest_distance = 0;

    for ((i, a), (j, b)) in points
        .iter()
        .enumerate()
        .tuple_combinations()
        .sorted_by_key(|((_, a), (_, b))| a.distance_squared(**b))
        .take(min_edges)
    {
        graph.add_edge(node_indices[i], node_indices[j], ());
        longest_distance = a.x * b.x;
        if connected_components(&graph) == 1 {
            break;
        }
    }

    (graph, longest_distance)
}

fn circuits(graph: &UnGraph<I64Vec3, ()>) -> Vec<Vec<I64Vec3>> {
    kosaraju_scc(graph)
        .into_iter()
        .sorted_by_key(|c| -(c.len() as i32))
        .map(|v| v.into_iter().map(|i| graph[i]).dedup().collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let input = input_generator(input);
        let (graph, _) = circuit_graph(&input, 10);
        let circuits = circuits(&graph);
        let result: usize = circuits.iter().map(|c| c.len()).take(3).product();
        assert_eq!(result, 40);
    }
}
