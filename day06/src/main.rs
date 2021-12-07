use std::{path::PathBuf, fs::canonicalize, time::Instant};

fn part_a() -> lib::error::Result<()> {
    let mut input_path = PathBuf::from("./day06/input.txt");
    let mut max_days: usize = 80;
    let mut args = std::env::args();
    args.next();

    if let Some(file_arg) = args.next() {
        input_path = canonicalize(&file_arg)?;

        if let Some(days_arg) = args.next() {
            if let Ok(days) = days_arg.parse() {
                max_days = days;
            } else {
                return Err(format!("failed to parse given max days. \"{}\"", days_arg).into());
            }
        }
    }

    let mut input = lib::read_file_lines(input_path)?;

    let mut school: [usize; 9] = [0; 9];
    let start = Instant::now();

    if let Some(line) = input.next() {
        if let Ok(ln) = line {
            for age in ln.split(',') {
                if let Ok(num) = age.parse::<usize>() {
                    if num > 8 {
                        return Err(format!("given number is too high. given: {}", num).into());
                    } else {
                        school[num] += 1;
                    }
                } else {
                    return Err(format!("failed to parse age of fish. given: {}", age).into());
                }
            }
        } else {
            return Err("failed to retrieve line from file".into())
        }
    } else {
        return Err("unexpected end of input.\nthe file must contain a single line of numbers delimitered by a ','".into())
    }

    let mut new_set: [usize; 9] = [0; 9];

    for _day in 0..max_days {
        for age in 0..9 {
            if age == 0 {
                new_set[6] += school[age];
                new_set[8] += school[age];
            } else {
                new_set[age-1] += school[age];
            }
        }

        school = new_set;

        for age in 0..9 {
            new_set[age] = 0;
        }
    }

    let mut count: usize = 0;

    for index in 0..9 {
        count += school[index];
    }

    let duration = start.elapsed();
    println!("result\n{}", count);
    lib::print_duration(&duration);

    Ok(())
}

fn main() {
    if let Err(e) = part_a() {
        println!("{}", e);
    }
}
