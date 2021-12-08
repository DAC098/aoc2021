use std::time::Instant;

fn calc_const_fuel(subs: &Vec<u16>, position: &u16, lowest: u64) -> u64 {
    let mut running: u64 = 0;

    for sub in subs {
        if *sub > *position {
            running += (*sub - *position) as u64;
        } else {
            running += (*position - *sub) as u64;
        }

        if running > lowest {
            return lowest;
        }
    }

    running
}

#[inline]
fn sum_positive_integers(value: u64) -> u64 {
    (value * (value + 1)) / 2
}

fn calc_non_const_fuel(subs: &Vec<u16>, position: &u16, lowest: u64) -> u64 {
    let mut running: u64 = 0;

    for sub in subs {
        if *sub > *position {
            running += sum_positive_integers((*sub - *position) as u64);
        } else {
            running += sum_positive_integers((*position - *sub) as u64);
        }

        if running > lowest {
            return lowest;
        }
    }

    running
}

fn part_a() -> lib::error::Result<()> {
    let mut input = lib::lines_from_arg_or_default("./day07/input.txt")?;
    let mut sub_positions: Vec<u16> = Vec::new();
    let mut largest_position: u16 = 0;
    let mut smallest_position: u16 = u16::MAX;

    let start = Instant::now();

    if let Some(line) = input.next() {
        if let Ok(ln) = line {

            for pos in ln.split(',') {
                if let Ok(num) = pos.parse::<u16>() {
                    if num > largest_position {
                        largest_position = num;
                    }

                    if num < smallest_position {
                        smallest_position = num;
                    }

                    sub_positions.push(num);
                } else {
                    return Err(format!("failed to parse num into unsigned integer: {}", pos).into());
                }
            }
        } else {
            return Err("failed to retrieve line from file".into())
        }
    } else {
        return Err("unexpected end of input. the file must contain a list of numbers delimitered by a ','".into());
    }

    let input_duration = start.elapsed();

    let mut smallest_const: u64 = u64::MAX;
    let mut smallest_non_const: u64 = u64::MAX;

    for pos in smallest_position..=largest_position {
        smallest_const = calc_const_fuel(&sub_positions, &pos, smallest_const);
        smallest_non_const = calc_non_const_fuel(&sub_positions, &pos, smallest_non_const);
    }

    let duration = start.elapsed();
    let diff = duration - input_duration;
    println!("part a\n{}\npart b\n{}", smallest_const, smallest_non_const);
    lib::print_duration(&diff);
    lib::print_duration(&duration);

    Ok(())
}

fn main() {
    if let Err(e) = part_a() {
        println!("{}", e);
    }
}
