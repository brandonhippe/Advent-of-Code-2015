use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn test_pw(pw: &Vec<u32>) -> bool {
    let disallowed: Vec<u32> = "iol".to_string().chars().map(|c| c as u32 - 'a' as u32).collect();
    if pw.iter().any(|n| disallowed.contains(n)) {
        return false;
    }

    let diffs: Vec<i64> = pw.windows(2).map(|nums| nums[1] as i64 - nums[0] as i64).collect();
    if !diffs.windows(2).any(|diffs| diffs[0] == 1 && diffs[1] == 1) {
        return false;
    }

    let zero_diffs: Vec<i64> = diffs.windows(2).filter(|ds| ds[1] == 0 && ds[0] != 0).map(|ds| ds[1]).collect();
    return zero_diffs.len() >= 2;
}

fn part1(contents: String) -> String {
    let mut pw_vec: Vec<u32> = contents.lines().next().unwrap().chars().map(|c| c as u32 - 'a' as u32).collect();
    while !test_pw(&pw_vec) {
        let mut ix = pw_vec.len() - 1;
        loop {
            pw_vec[ix] += 1;
            pw_vec[ix] %= 26;
            if pw_vec[ix] == 0 {
                ix -= 1;
            } else {
                break;
            }
        }
    }

    return pw_vec.iter().fold("".to_string(), |pw, &c| format!("{}{}", pw, char::from_u32(c + 'a' as u32).unwrap()));
}

fn part2(contents: String) -> String {
    let new_start = part1(contents);
    let mut pw_vec: Vec<u32> = new_start.lines().next().unwrap().chars().map(|c| c as u32 - 'a' as u32).collect();
    let mut first_ran: bool = false;
    
    while !first_ran || !test_pw(&pw_vec) {
        first_ran = true;
        let mut ix = pw_vec.len() - 1;
        loop {
            pw_vec[ix] += 1;
            pw_vec[ix] %= 26;
            if pw_vec[ix] == 0 {
                ix -= 1;
            } else {
                break;
            }
        }
    }

    return pw_vec.iter().fold("".to_string(), |pw, &c| format!("{}{}", pw, char::from_u32(c + 'a' as u32).unwrap()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 0);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 0);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2015".to_string();
    let day = "11".to_string();

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
        "\nPart 1:\nNext password: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nNext password: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}