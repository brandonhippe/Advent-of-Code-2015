use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

#[derive(Clone, Debug)]
struct Program<'a> {
    instructions: Vec<&'a str>,
    pc: i64,
    reg_a: u64,
    reg_b: u64,
}

impl Default for Program<'_> {
    fn default() -> Program<'static> {
        Program {
            instructions: vec![],
            pc: 0,
            reg_a: 0,
            reg_b: 0,
        }
    }
}

impl Program<'_> {
    fn run(&mut self) -> u64 {
        while 0 <= self.pc && (self.pc as usize) < self.instructions.len() {
            self.execute_instruction();
        }
        
        return self.reg_b;
    }
    
    fn execute_instruction(&mut self) {
        let ins = self.instructions[self.pc as usize];
        let ins_words: Vec<&str> = ins.split_whitespace().collect();

        let reg_val: Option<&mut u64> = match ins_words[1].chars().next().unwrap() {
            'a' => Some(&mut self.reg_a),
            'b' => Some(&mut self.reg_b),
            _ => None
        };
        
        match ins.split_whitespace().next().unwrap() {
            "hlf" => {
                *reg_val.unwrap() /= 2;
                self.pc += 1;
            },
            "tpl" => {
                *reg_val.unwrap() *= 3;
                self.pc += 1;
            },
            "inc" => {
                *reg_val.unwrap() += 1;
                self.pc += 1;
            },
            "jmp" => {
                self.pc += ins_words[1].parse::<i64>().unwrap();
            },
            "jie" => {
                self.pc += if *reg_val.unwrap() % 2 == 0 {
                    ins_words[2].parse::<i64>().unwrap()
                } else {
                    1
                }
            },
            "jio" => {
                self.pc += if *reg_val.unwrap() == 1 {
                    ins_words[2].parse::<i64>().unwrap()
                } else {
                    1
                }
            },
            _ => panic!("Unknown instruction!")
        }
    }
}

fn part1(contents: String) -> u64 {
    return Program {
        instructions: contents.lines().map(|l| l.trim()).collect(),
        ..Default::default()
    }.run();
}

fn part2(contents: String) -> u64 {
    return Program {
        instructions: contents.lines().map(|l| l.trim()).collect(),
        reg_a: 1,
        ..Default::default()
    }.run();
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
    let day = "23".to_string();

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
        "\nPart 1:\nRegister B: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nRegister B: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}