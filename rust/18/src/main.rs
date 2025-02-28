use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{HashSet, HashMap};
use itertools::Itertools;

fn iterate(on_lights: HashSet<(i64, i64)>, dim: i64) -> HashSet<(i64, i64)> {
    let mut neighbor_counts: HashMap<(i64, i64), i64> = HashMap::new();
    for (x, y) in on_lights.iter() {
        for (x_off, y_off) in (-1..=1).cartesian_product(-1..=1) {
            if x_off != 0 || y_off != 0 {
                let new_pos = (x + x_off, y + y_off);
                *neighbor_counts.entry(new_pos).or_insert(0) += 1;
            }
        }
    }

    return HashSet::from_iter(neighbor_counts.iter().filter_map(|(pos, count)| {
        if pos.0 >= 0 && pos.0 < dim && pos.1 >= 0 && pos.1 < dim {
            if *count == 3 || (on_lights.contains(pos) && *count == 2) {
                Some(*pos)
            } else {
                None
            }
        } else {
            None
        }
    }));
}

fn part1(contents: String, steps: i64) -> i64 {
    let dim: i64 = contents.lines().count() as i64;
    let mut on_lights = HashSet::new();
    for (y, line) in contents.lines().enumerate() {
        for x in line.chars().enumerate().filter_map(|(x, c)| {
            match c {
                '#' => Some(x),
                _ => None
            }
        }) {
            on_lights.insert((x as i64, y as i64));
        }
    }

    for _ in 0..steps {
        on_lights = iterate(on_lights, dim);
    }
    return on_lights.len() as i64;
}

fn part2(contents: String, steps: i64) -> i64 {
    let dim: i64 = contents.lines().count() as i64;
    let mut on_lights = HashSet::from_iter((0..dim).step_by(dim as usize - 1).cartesian_product((0..dim).step_by(dim as usize - 1)));
    for (y, line) in contents.lines().enumerate() {
        for x in line.chars().enumerate().filter_map(|(x, c)| {
            match c {
                '#' => Some(x),
                _ => None
            }
        }) {
            on_lights.insert((x as i64, y as i64));
        }
    }
    let corners: HashSet<(i64, i64)> = HashSet::from_iter((0..dim).step_by(dim as usize - 1).cartesian_product((0..dim).step_by(dim as usize - 1)));

    for _ in 0..steps {
        on_lights = corners.union(&iterate(on_lights, dim as i64)).map(|p| *p).collect();
    }
    return on_lights.len() as i64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents, 4), 4);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents, 5), 17);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2015".to_string();
    let day = "18".to_string();

    let root = env::current_dir().unwrap();
    let path_str = if args.len() > 1 {
        args[1].clone()
    } else if root.ends_with(format!("{}", day)) {
        format!("../../../Inputs/{}_{}.txt", year, day)
    } else {
        format!("/Inputs/{}_{}.txt", year, day)
    };

    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nLights on: {}\nRan in {:.5?}",
        part1(contents.clone(), 100),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nLights on: {}\nRan in {:.5?}",
        part2(contents.clone(), 100),
        part2_timer.elapsed()
    );
}