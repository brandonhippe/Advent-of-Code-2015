use relative_path::RelativePath;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    return contents
        .chars()
        .fold(
            ((0, 0), HashSet::from([(0, 0)])),
            |(pos, mut visited), c| {
                let new_pos = match c {
                    '^' => (pos.0, pos.1 + 1),
                    'v' => (pos.0, pos.1 - 1),
                    '<' => (pos.0 - 1, pos.1),
                    '>' => (pos.0 + 1, pos.1),
                    _ => pos,
                };

                visited.insert(new_pos);
                (new_pos, visited)
            },
        )
        .1
        .len() as i64;
}

fn part2(contents: String) -> i64 {
    return contents
        .chars()
        .fold(
            ((0, 0), (0, 0), 0, HashSet::from([(0, 0)])),
            |(santa, robot, ix, mut visited), c| {
                if ix == 0 {

                    let new_pos = match c {
                        '^' => (santa.0, santa.1 + 1),
                        'v' => (santa.0, santa.1 - 1),
                        '<' => (santa.0 - 1, santa.1),
                        '>' => (santa.0 + 1, santa.1),
                        _ => santa,
                    };

                    visited.insert(new_pos);
                    (new_pos, robot, 1, visited)
                } else {
                    let new_pos = match c {
                        '^' => (robot.0, robot.1 + 1),
                        'v' => (robot.0, robot.1 - 1),
                        '<' => (robot.0 - 1, robot.1),
                        '>' => (robot.0 + 1, robot.1),
                        _ => robot,
                    };

                    visited.insert(new_pos);
                    (santa, new_pos, 0, visited)
                }
            },
        )
        .3
        .len() as i64;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2015".to_string();
    let day = "3".to_string();

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
        "\nPart 1:\nHouses that receive presents: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nHouses that receive presents: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}