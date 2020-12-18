use std::collections::{BTreeSet, HashSet};

use crate::conway_cubes::{parse_cubes, MinMax};

type CubeMap4d = HashSet<CubePos4d>;

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct CubePos4d {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

#[derive(PartialEq, Eq, Debug)]
struct CubeMinMax4d {
    x: MinMax,
    y: MinMax,
    z: MinMax,
    w: MinMax,
}

/// Part2
fn active_cubes_after_boot_4d(initial_cubes: Vec<String>) -> usize {
    let boot_runs = 6;
    let active_cubes_map = parse_cubes_4d(initial_cubes);

    let map_after = run_cycles_4d(active_cubes_map, boot_runs);

    map_after.into_iter().count()
}

fn run_cycles_4d(cube_map: CubeMap4d, boot_runs: usize) -> CubeMap4d {
    if boot_runs == 0 {
        return cube_map;
    }
    let mut new_map: CubeMap4d = HashSet::new();
    let min_maxes = get_min_max_4d(&cube_map);

    for x in min_maxes.x.min - 1..=min_maxes.x.max + 1 {
        for y in min_maxes.y.min - 1..=min_maxes.y.max + 1 {
            for z in min_maxes.z.min - 1..=min_maxes.z.max + 1 {
                for w in min_maxes.w.min - 1..=min_maxes.w.max + 1 {
                    let pos = CubePos4d { x: x, y: y, z: z, w: w, };
                    let neighbors = get_neighbors_4d(&pos);
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
    }

    run_cycles_4d(new_map, boot_runs - 1)
}

fn get_neighbors_4d(pos: &CubePos4d) -> Vec<CubePos4d> {
    let mut neighbors = vec![];

    for x in (pos.x - 1)..=(pos.x + 1) {
        for y in (pos.y - 1)..=(pos.y + 1) {
            for z in (pos.z - 1)..=(pos.z + 1) {
                for w in (pos.w - 1)..=(pos.w + 1) {
                neighbors.push(CubePos4d { x: x, y: y, z: z, w: w });
                }
            }
        }
    }

    neighbors
        .into_iter()
        .filter(|neighbor| *neighbor != *pos)
        .collect()
}

fn get_min_max_4d(cube_map: &HashSet<CubePos4d>) -> CubeMinMax4d {
    let each_x: BTreeSet<isize> = BTreeSet::new();
    let each_y: BTreeSet<isize> = BTreeSet::new();
    let each_z: BTreeSet<isize> = BTreeSet::new();
    let each_w: BTreeSet<isize> = BTreeSet::new();
    let eaches = cube_map
        .iter()
        .fold((each_x, each_y, each_z, each_w), |mut min_max, pos| {
            min_max.0.insert(pos.x);
            min_max.1.insert(pos.y);
            min_max.2.insert(pos.z);
            min_max.3.insert(pos.w);

            min_max
        });

    CubeMinMax4d {
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
        w: MinMax {
            min: eaches.3.iter().nth(0).unwrap().clone(),
            max: eaches.3.iter().last().unwrap().clone(),
        },
    }
}

fn parse_cubes_4d(initial_cubes: Vec<String>) -> HashSet<CubePos4d> {
    parse_cubes(initial_cubes)
        .into_iter()
        .map(|pos| CubePos4d {
            x: pos.x,
            y: pos.y,
            z: 0,
            w: 0,
        })
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
    fn active_cubes_after_boot_4d_example() {
        let expected = 848;
        let cubes = mutliline_to_vec_string(EXAMPLE_CUBES.to_string());
        let actual = active_cubes_after_boot_4d(cubes);

        assert_eq!(actual, expected);
    }

    #[test]
    fn get_min_max_4d_test() {
        let expected = CubeMinMax4d {
            x: MinMax { min: -2, max: 4 },
            y: MinMax { min: -5, max: 5 },
            z: MinMax { min: 0, max: 0 },
            w: MinMax { min: -2, max: 2 },
        };
        let mut cubes: HashSet<CubePos4d> = HashSet::new();
        cubes.insert(CubePos4d {
            x: -1,
            y: 5,
            z: 0,
            w: 2,
        });
        cubes.insert(CubePos4d {
            x: 4,
            y: 0,
            z: 0,
            w: -2,
        });
        cubes.insert(CubePos4d {
            x: -2,
            y: -5,
            z: 0,
            w: 0,
        });
        let actual = get_min_max_4d(&cubes);

        assert_eq!(actual, expected);
    }

    #[test]
    fn get_neighbors_4d_simple_test() {
        let expected_len = 80;
        let pos = CubePos4d {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        };
        let actual = get_neighbors_4d(&pos);

        assert_eq!(actual.len(), expected_len);
        assert!(!actual.contains(&pos));
    }

    #[test]
    fn parse_cubes_4d_test() {
        let mut expected: HashSet<CubePos4d> = HashSet::new();
        expected.insert(CubePos4d {
            x: 0,
            y: 1,
            z: 0,
            w: 0,
        });
        expected.insert(CubePos4d {
            x: 1,
            y: 2,
            z: 0,
            w: 0,
        });
        expected.insert(CubePos4d {
            x: 2,
            y: 0,
            z: 0,
            w: 0,
        });
        expected.insert(CubePos4d {
            x: 2,
            y: 1,
            z: 0,
            w: 0,
        });
        expected.insert(CubePos4d {
            x: 2,
            y: 2,
            z: 0,
            w: 0,
        });

        let cubes = mutliline_to_vec_string(EXAMPLE_CUBES.to_string());
        let actual = parse_cubes_4d(cubes);

        assert_eq!(actual, expected);
    }

    // Part2
    #[test]
    fn active_cubes_after_boot_4d_from_input() {
        let expected = 1980;

        let cubes = load_as_vec_string("day17");
        let actual = active_cubes_after_boot_4d(cubes);
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
