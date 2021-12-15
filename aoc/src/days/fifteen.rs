use libaoc::{Day, DayNumber};
use petgraph::{algo::dijkstra, prelude::NodeIndex, Directed, Graph};

fn shortest_path(input: &[Vec<usize>]) -> usize {
    let mut graph = Graph::<usize, usize, Directed>::new();
    let height = input.len();
    let width = input[0].len();
    let nodes: Vec<NodeIndex> = input
        .iter()
        .flatten()
        .map(|risk_level| graph.add_node(*risk_level))
        .collect();

    for i in 0..height {
        for j in 0..width {
            let node_index = nodes[i * width + j];
            if j > 0 {
                let neighbor_index = nodes[i * width + j - 1];
                graph.add_edge(
                    node_index,
                    neighbor_index,
                    *graph.node_weight(neighbor_index).unwrap(),
                );
            }
            if j < width - 1 {
                let neighbor_index = nodes[i * width + j + 1];
                graph.add_edge(
                    node_index,
                    neighbor_index,
                    *graph.node_weight(neighbor_index).unwrap(),
                );
            }
            if i > 0 {
                let neighbor_index = nodes[(i - 1) * width + j];
                graph.add_edge(
                    node_index,
                    neighbor_index,
                    *graph.node_weight(neighbor_index).unwrap(),
                );
            }
            if i < height - 1 {
                let neighbor_index = nodes[(i + 1) * width + j];
                graph.add_edge(
                    node_index,
                    neighbor_index,
                    *graph.node_weight(neighbor_index).unwrap(),
                );
            }
        }
    }

    let map = dijkstra(&graph, nodes[0], Some(nodes[nodes.len() - 1]), |e| {
        *e.weight()
    });

    *map.get(&nodes[nodes.len() - 1]).unwrap()
}

fn stitch_map(map: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let height = map.len();

    let mut map_col_5: Vec<Vec<usize>> = map.to_vec();
    for i in 0..height {
        for j in 1..5 {
            map_col_5[i].append(
                &mut map[i]
                    .iter()
                    .map(|risk_level| {
                        if *risk_level + j >= 10 {
                            (*risk_level + j) % 10 + 1
                        } else {
                            *risk_level + j
                        }
                    })
                    .collect::<Vec<usize>>(),
            );
        }
    }

    let mut map_row_col_5 = map_col_5.clone();
    for j in 1..5 {
        map_row_col_5.append(
            &mut map_col_5
                .iter()
                .map(|line| {
                    line.iter()
                        .map(|risk_level| {
                            if *risk_level + j >= 10 {
                                (*risk_level + j) % 10 + 1
                            } else {
                                *risk_level + j
                            }
                        })
                        .collect()
                })
                .collect(),
        )
    }
    map_row_col_5
}

pub fn fifteen() -> Day<2021> {
    Day::new(
        DayNumber::Fifteen,
        |input| {
            let map: Vec<Vec<usize>> = input
                .lines()
                .map(|line| {
                    line.bytes()
                        .map(|byte| (byte - b'0') as usize)
                        .collect::<Vec<usize>>()
                })
                .collect();

            Box::new(shortest_path(&map))
        },
        |input| {
            let map: Vec<Vec<usize>> = input
                .lines()
                .map(|line| {
                    line.bytes()
                        .map(|byte| (byte - b'0') as usize)
                        .collect::<Vec<usize>>()
                })
                .collect();

            Box::new(shortest_path(&stitch_map(&map)))
        },
    )
}

#[cfg(test)]
mod tests {
    use crate::days::fifteen::stitch_map;

    use super::shortest_path;

    #[test]
    fn example() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

        let map: Vec<Vec<usize>> = input
            .lines()
            .map(|line| {
                line.bytes()
                    .map(|byte| (byte - b'0') as usize)
                    .collect::<Vec<usize>>()
            })
            .collect();

        assert_eq!(shortest_path(&map), 40);
        assert_eq!(shortest_path(&stitch_map(&map)), 315);
    }
}
