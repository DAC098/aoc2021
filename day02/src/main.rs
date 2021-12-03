use std::time::Instant;

fn part_a() -> lib::ResultBoxE<()> {
    let input = lib::lines_from_arg_or_default("./day02/input.txt")?;

    let mut depth: u64 = 0;
    let mut traveled: u64 = 0;
    let start = Instant::now();

    for line in input {
        if let Ok(ln) = line {
            if let Some((direction, value)) = ln.split_once(' ') {
                let num: u64;

                if let Ok(v) = value.parse() {
                    num = v;
                } else {
                    println!("failed to parse value to u64. given: {}", value);
                    break;
                }

                match direction {
                    "forward" => traveled += num,
                    "up" => depth -= num,
                    "down" => depth += num,
                    _ => {
                        println!("invalid direction given. accepted: \"forward\", \"up\", \"down\" given: \"{}\"", direction);
                        break;
                    }
                }
            } else {
                println!("invalid input given. expecting a direction and a value split by a single white space. given: {}", ln);
                break;
            }
        } else {
            println!("failed to retrieve line");
            break;
        }
    }

    let duration = start.elapsed();
    println!("part a\n{}", depth * traveled);
    lib::print_duration(&duration);

    Ok(())
}

fn part_b() -> lib::ResultBoxE<()> {
    let input = lib::lines_from_arg_or_default("./day02/input.txt")?;

    let mut depth: i64 = 0;
    let mut traveled: i64 = 0;
    let mut aim: i64 = 0;
    let start = Instant::now();

    for line in input {
        if let Ok(ln) = line {
            if let Some((direction, value)) = ln.split_once(' ') {
                let num: i64;

                if let Ok(v) = value.parse() {
                    num = v;
                } else {
                    println!("failed to parse value to i64. given: {}", value);
                    break;
                }

                match direction {
                    "forward" => {
                        traveled += num;
                        depth += aim * num;
                    },
                    "up" => aim -= num,
                    "down" => aim += num,
                    _ => {
                        println!("invalid direction given. accepted: \"forward\", \"up\", \"down\" given: \"{}\"", direction);
                        break;
                    }
                }
            } else {
                println!("invalid input given. expecting a direction and a value split by a single white space. given: {}", ln);
                break;
            }
        } else {
            println!("failed to retrieve line");
            break;
        }
    }

    let duration = start.elapsed();
    println!("part b\n{}", depth * traveled);
    lib::print_duration(&duration);

    Ok(())
}

fn main() {
    if let Err(e) = part_a() {
        println!("{}", e.to_string());
    }

    if let Err(e) = part_b() {
        println!("{}", e.to_string());
    }
}
