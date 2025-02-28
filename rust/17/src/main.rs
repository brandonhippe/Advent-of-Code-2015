use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;
use std::collections::{HashMap, VecDeque};

fn fill_containers(mut containers: VecDeque<i64>, store: i64, num_used: i64) -> HashMap<i64, i64> {
    if store == 0 {
        return HashMap::from([(num_used, 1)]);
    }

    let mut working_fills: HashMap<i64, i64> = HashMap::new();
    while let Some(cont) = containers.pop_front() {
        for (cont_amt, count) in fill_containers(containers.clone(), store - cont, num_used + 1) {
            *working_fills.entry(cont_amt).or_insert(0) += count;
        }
    }

    return working_fills;
}

fn part1(contents: String, store: i64) -> i64 {
    let mut containers: Vec<i64> = contents.lines().map(|l| l.trim().parse::<i64>().unwrap()).collect();
    containers.sort_by(|a, b| b.cmp(&a));
    return fill_containers(VecDeque::from(containers), store, 0).values().sum::<i64>();
}

fn part2(contents: String, store: i64) -> i64 {
    let mut containers: Vec<i64> = contents.lines().map(|l| l.trim().parse::<i64>().unwrap()).collect();
    containers.sort_by(|a, b| b.cmp(&a));
    return fill_containers(VecDeque::from(containers), store, 0).iter().min_by(|e1, e2| e1.0.cmp(e2.0)).map(|(_k, v)| *v).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents, 25), 4);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents, 25), 3);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2015".to_string();
    let day = "17".to_string();

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
        "\nPart 1:\nCombinations: {}\nRan in {:.5?}",
        part1(contents.clone(), 150),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMinimal combinations: {}\nRan in {:.5?}",
        part2(contents.clone(), 150),
        part2_timer.elapsed()
    );
}