use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut wires: Vec<Wire> = Vec::new();
    for line in contents.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let name = parts[1].to_string();
        let input = parts[0].to_string();
        wires.push(Wire::new(name, input));
    }

    wires.sort_by(|a, b| a.name.cmp(&b.name));

    while wires[0].value.is_none() {
        for ix in 0..wires.len() {
            if let Some(value) = wires[ix].evaluate(ix, &wires) {
                wires[ix].value = Some(value);
            }
        }
    }

    return wires[0].value.unwrap() as i64;
}

fn part2(contents: String) -> i64 {
    let a_value = part1(contents.clone());
    let mut wires: Vec<Wire> = Vec::new();

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let name = parts[1].to_string();
        let input = if name == "b" {

            a_value.to_string()
        } else {
            parts[0].to_string()
        };

        wires.push(Wire::new(name, input));
    }

    wires.sort_by(|a, b| a.name.cmp(&b.name));

    while wires[0].value.is_none() {
        for ix in 0..wires.len() {
            if let Some(value) = wires[ix].evaluate(ix, &wires) {
                wires[ix].value = Some(value);
            }
        }
    }

    return wires[0].value.unwrap() as i64;
}

#[derive(Debug, Clone)]
struct Wire {
    name: String,
    value: Option<u16>,
    op: Option<String>,
}

impl Wire {
    fn new(name: String, input: String) -> Wire {
        Wire {
            name,
            value: if input.parse::<u16>().is_ok() {
                Some(input.parse().unwrap())
            } else {
                None
            },
            op: if input.parse::<u16>().is_err() {
                Some(input)
            } else {
                None
            },
        }
    }

    fn evaluate(&self, self_ix: usize, wires: &Vec<Wire>) -> Option<u16> {
        if self.value.is_some() {
            return self.value;
        }

        let op = self.op.as_ref().unwrap();
        let parts: Vec<&str> = op.split(" ").collect();
        let mut inputs: Vec<u16> = Vec::new();
        for part in &parts {
            if part.parse::<u16>().is_ok() {
                inputs.push(part.parse().unwrap());
            } else if !(*part == "AND"
                || *part == "OR"
                || *part == "LSHIFT"
                || *part == "RSHIFT"
                || *part == "NOT")
            {
                let wire_ix = wires.iter().position(|x| x.name == *part).unwrap();
                if wire_ix != self_ix && wires[wire_ix].value.is_some() {
                    inputs.push(wires[wire_ix].value.unwrap());
                } else {
                    return None;
                }
            }
        }

        let result = match parts.len() {
            1 => inputs[0],
            2 => match parts[0] {
                "NOT" => !inputs[0],
                _ => panic!("Unknown operation"),
            },
            _ => match parts[1] {
                "AND" => inputs[0] & inputs[1],
                "OR" => inputs[0] | inputs[1],
                "LSHIFT" => inputs[0] << inputs[1],
                "RSHIFT" => inputs[0] >> inputs[1],
                _ => panic!("Unknown operation"),
            },
        };

        return Some(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 72);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let year = "2015".to_string();
    let day = "7".to_string();

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
        "\nPart 1:\nSignal provided to wire a: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSignal provided to wire a: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}