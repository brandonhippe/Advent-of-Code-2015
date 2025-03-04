use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use itertools::Itertools;
use cached::proc_macro::cached;

#[cached]
fn splitable(vals: Vec<i64>, goal: i64, groups: i64) -> bool {
    for comb_len in 1..=vals.len() {
        for poss_group in vals.iter().combinations(comb_len).filter(|v| v.iter().map(|val| *val).sum::<i64>() == goal) {
            if groups == 2 {
                return true;
            } else {
                return splitable(vals.iter().filter_map(|v| if !poss_group.contains(&v) { Some(*v) } else { None }).collect(), goal, groups - 1);
            }
        }
    }

    return false;
}

fn part1(contents: String) -> i64 {
    let package_weights: Vec<i64> = contents.lines().map(|l| l.parse::<i64>().unwrap()).collect();
    let goal_weight: i64 = package_weights.iter().sum::<i64>() / 3;

    let mut min_val: i64 = i64::MAX;
    for comb_len in 1..=package_weights.len() {
        for poss_group in package_weights.iter().combinations(comb_len).filter(|v| {
            v.iter().map(|val| *val).sum::<i64>() == goal_weight && splitable(package_weights.iter().filter_map(|w| if !v.contains(&w) { Some(*w) } else { None }).collect(), goal_weight, 2)
        }) {
            min_val = min_val.min(poss_group.iter().map(|v| *v).product::<i64>());
        }
        if min_val < i64::MAX {
            break;
        }
    }

    return min_val;
}

fn part2(contents: String) -> i64 {
    let package_weights: Vec<i64> = contents.lines().map(|l| l.parse::<i64>().unwrap()).collect();
    let goal_weight: i64 = package_weights.iter().sum::<i64>() / 4;

    let mut min_val: i64 = i64::MAX;
    for comb_len in 1..=package_weights.len() {
        for poss_group in package_weights.iter().combinations(comb_len).filter(|v| {
            v.iter().map(|val| *val).sum::<i64>() == goal_weight && splitable(package_weights.iter().filter_map(|w| if !v.contains(&w) { Some(*w) } else { None }).collect(), goal_weight, 3)
        }) {
            min_val = min_val.min(poss_group.iter().map(|v| *v).product::<i64>());
        }
        if min_val < i64::MAX {
            break;
        }
    }

    return min_val;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 99);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 44);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2015".to_string();
    let day = "24".to_string();

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
        "\nPart 1:\nLowest quantum entanglement: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nLowest quantum entanglement: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}