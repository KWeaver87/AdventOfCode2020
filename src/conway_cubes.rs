use std::collections::{BTreeSet, HashSet};

pub type CubeMap = HashSet<CubePos>;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
pub struct CubePos {
    pub x: isize,
    pub y: isize,
    pub z: isize,
}

#[derive(PartialEq, Eq, Debug)]
pub struct MinMax {
    pub min: isize,
    pub max: isize,
}

#[derive(PartialEq, Eq, Debug)]
struct CubeMinMax {
    x: MinMax,
    y: MinMax,
    z: MinMax,
}

/// Part1
fn active_cubes_after_boot(initial_cubes: Vec<String>) -> usize {
    let boot_runs = 6;
    let active_cubes_map = parse_cubes(initial_cubes);

    let map_after = run_cycles(active_cubes_map, boot_runs);

    map_after.into_iter().count()
}

fn run_cycles(cube_map: CubeMap, boot_runs: usize) -> CubeMap {
    if boot_runs == 0 {
        return cube_map;
    }
    let mut new_map: CubeMap = HashSet::new();
    let min_maxes = get_min_max(&cube_map);

    for x in min_maxes.x.min - 1..=min_maxes.x.max + 1 {
        for y in min_maxes.y.min - 1..=min_maxes.y.max + 1 {
            for z in min_maxes.z.min - 1..=min_maxes.z.max + 1 {
                let pos = CubePos { x: x, y: y, z: z };
                let neighbors = get_neighbors(&pos);
                let active_neighbor_count = neighbors
                    .into_iter()
                    .filter(|p| cube_map.contains(p))
                    .count();

                if active_neighbor_count == 3 {
                    new_map.insert(pos);
                } else if active_neighbor_count == 2 && cube_map.contains(&pos) {
                    new_map.insert(pos);
                }
            }
        }
    }

    run_cycles(new_map, boot_runs - 1)
}

fn get_neighbors(pos: &CubePos) -> Vec<CubePos> {
    let mut neighbors = vec![];

    for x in (pos.x - 1)..=(pos.x + 1) {
        for y in (pos.y - 1)..=(pos.y + 1) {
            for z in (pos.z - 1)..=(pos.z + 1) {
                neighbors.push(CubePos { x: x, y: y, z: z });
            }
        }
    }

    neighbors
        .into_iter()
        .filter(|neighbor| *neighbor != *pos)
        .collect()
}

fn get_min_max(cube_map: &HashSet<CubePos>) -> CubeMinMax {
    let each_x: BTreeSet<isize> = BTreeSet::new();
    let each_y: BTreeSet<isize> = BTreeSet::new();
    let each_z: BTreeSet<isize> = BTreeSet::new();
    let eaches = cube_map
        .iter()
        .fold((each_x, each_y, each_z), |mut min_max, pos| {
            min_max.0.insert(pos.x);
            min_max.1.insert(pos.y);
            min_max.2.insert(pos.z);

            min_max
        });

    CubeMinMax {
        x: MinMax {
            min: eaches.0.iter().nth(0).unwrap().clone(),
            max: eaches.0.iter().last().unwrap().clone(),
        },
        y: MinMax {
            min: eaches.1.iter().nth(0).unwrap().clone(),
            max: eaches.1.iter().last().unwrap().clone(),
        },
        z: MinMax {
            min: eaches.2.iter().nth(0).unwrap().clone(),
            max: eaches.2.iter().last().unwrap().clone(),
        },
    }
}

pub fn parse_cubes(initial_cubes: Vec<String>) -> HashSet<CubePos> {
    initial_cubes
        .iter()
        .enumerate()
        .map(|(x, row)| {
            row.chars()
                .enumerate()
                .filter(|(_, ch)| ch == &'#')
                .map(|(y, _)| CubePos {
                    x: x as isize,
                    y: y as isize,
                    z: 0 as isize,
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::input_utils::{load_as_vec_string, mutliline_to_vec_string};
    use colored::Colorize;

    static EXAMPLE_CUBES: &str = ".#.
..#
###";

    #[test]
    fn active_cubes_after_boot_example() {
        let expected = 112;
        let cubes = mutliline_to_vec_string(EXAMPLE_CUBES.to_string());
        let actual = active_cubes_after_boot(cubes);

        assert_eq!(actual, expected);
    }

    #[test]
    fn get_min_max_test() {
        let expected = CubeMinMax {
            x: MinMax { min: -2, max: 4 },
            y: MinMax { min: -5, max: 5 },
            z: MinMax { min: 0, max: 0 },
        };
        let mut cubes: HashSet<CubePos> = HashSet::new();
        cubes.insert(CubePos { x: -1, y: 5, z: 0 });
        cubes.insert(CubePos { x: 4, y: 0, z: 0 });
        cubes.insert(CubePos { x: -2, y: -5, z: 0 });
        let actual = get_min_max(&cubes);

        assert_eq!(actual, expected);
    }

    #[test]
    fn get_neighbors_simple_test() {
        let expected_len = 26;
        let pos = CubePos { x: 0, y: 0, z: 0 };
        let actual = get_neighbors(&pos);

        assert_eq!(actual.len(), expected_len);
        assert!(!actual.contains(&pos));
    }

    #[test]
    fn parse_cubes_test() {
        let mut expected: HashSet<CubePos> = HashSet::new();
        expected.insert(CubePos { x: 0, y: 1, z: 0 });
        expected.insert(CubePos { x: 1, y: 2, z: 0 });
        expected.insert(CubePos { x: 2, y: 0, z: 0 });
        expected.insert(CubePos { x: 2, y: 1, z: 0 });
        expected.insert(CubePos { x: 2, y: 2, z: 0 });

        let cubes = mutliline_to_vec_string(EXAMPLE_CUBES.to_string());
        let actual = parse_cubes(cubes);

        assert_eq!(actual, expected);
    }

    // Part1
    #[test]
    fn active_cubes_after_boot_from_input() {
        let expected = 362;

        let cubes = load_as_vec_string("day17");
        let actual = active_cubes_after_boot(cubes);
        println!(
            "{}{}",
            "Number of active cubes after 6 boot cycles: "
                .green()
                .bold(),
            actual
        );

        assert_eq!(actual, expected);
    }
}
