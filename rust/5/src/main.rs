use itertools::Itertools;
use regex::Regex;
use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let vowel_re: Regex = Regex::new(r"([aeiou])").unwrap();
    let bad_re: Regex = Regex::new(r"(ab|cd|pq|xy)").unwrap();
    let double_re: Regex = Regex::new(
        "(aa|bb|cc|dd|ee|ff|gg|hh|ii|jj|kk|ll|mm|nn|oo|pp|qq|rr|ss|tt|uu|vv|ww|xx|yy|zz)",
    )
    .unwrap();
    return contents
        .lines()
        .map(|line| {
            if !bad_re.is_match(line)
                && vowel_re.find_iter(line).count() >= 3
                && double_re.is_match(line)
            {
                return 1;
            }
            return 0;
        })
        .sum::<i64>();
}

fn part2(contents: String) -> i64 {
    let mut pair_string = String::new();
    for a in 'a'..='z' {
        for b in 'a'..='z' {
            pair_string.push_str(&format!("{}{}.*{}{}|", a, b, a, b));
        }
    }


    pair_string.pop();

    let pair_re: Regex = Regex::new(&pair_string).unwrap();
    let repeat_re: Regex = Regex::new("a.a|b.b|c.c|d.d|e.e|f.f|g.g|h.h|i.i|j.j|k.k|l.l|m.m|n.n|o.o|p.p|q.q|r.r|s.s|t.t|u.u|v.v|w.w|x.x|y.y|z.z").unwrap();
    return contents
        .lines()
        .map(|line| {
            if pair_re.is_match(line) && repeat_re.is_match(line) {
                return 1;
            }
            return 0;
        })
        .sum::<i64>();
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2015".to_string();
    let day = "5".to_string();

    let root = env::current_dir().unwrap();
    let path_str = if args.len() > 1 {
        args[1].clone()
    } else if root.ends_with(format!("rust_{}_{}", year, day)) {
        format!("../../../Inputs/{}_{}.txt", year, day)
    } else {
        format!("/Inputs/{}_{}.txt", year, day)
    };


    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nNice Strings: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nNice Strings: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}