use std::{time::Instant};

const UNICODE_ONE: u8 = 49;

fn part_a() -> lib::ResultBoxE<()> {
    let mut input = lib::lines_from_arg_or_default("./day03/input.txt")?.peekable();

    let mut on_count: Vec<u32> = Vec::new();
    let mut off_count: Vec<u32> = Vec::new();

    let start = Instant::now();

    if let Some(line) = input.peek() {
        if let Ok(ln) = line {
            for _ in 0..ln.chars().count() {
                on_count.push(0);
                off_count.push(0)
            }
        }
    }

    for line in input {
        if let Ok(ln) = line {
            let mut index: usize = 0;

            for ch in ln.chars() {
                match ch {
                    '1' => {
                        if let Some(count) = on_count.get_mut(index) {
                            *count += 1;
                        } else {
                            println!("unexpected line size. all input lines must be the same length. line: {}", ln);
                            return Ok(());
                        }
                    },
                    '0' => {
                        if let Some(count) = off_count.get_mut(index) {
                            *count += 1;
                        } else {
                            println!("unexpected line size. all input lines must be the same length. line: {}", ln);
                            return Ok(());
                        }
                    },
                    _ => {
                        println!("unexpected character given in input. all characters must 1 or 0. line: {}", ln);
                        return Ok(())
                    }
                }

                index += 1;
            }
        } else {
            println!("failed to retrieve line from input");
            break;
        }
    }

    let mut gamma: u64 = 0;
    let mut epsilon: u64 = 0;
    let col_count = on_count.len();

    for index in 0..col_count {
        let mask = 1 << (col_count - 1 - index);
        let on_value = on_count.get(index).unwrap();
        let off_value = off_count.get(index).unwrap();

        if on_value > off_value {
            gamma |= mask;
        } else {
            epsilon |= mask;
        }
    }

    let duration = start.elapsed();
    println!("part a\n{}", gamma * epsilon);
    lib::print_duration(&duration);

    Ok(())
}

fn u8_slice_to_u64(list: &Vec<u8>) -> u64 {
    let mut rtn: u64 = 0;
    let adjust = list.len() - 1;

    for index in 0..list.len() {
        if list[index] == UNICODE_ONE {
            rtn |= 1 << (adjust - index);
        }
    }

    rtn
}

enum FindOp {
    MostCommon,
    LeastCommon
}

fn find_value(column_size: usize, list: &Vec<Vec<u8>>, op: FindOp) -> Result<u64, String> {
    let mut column_index: usize = 0;
    let mut working_list: Vec<&Vec<u8>> = list.iter().map(|f| f).collect();

    /*
    print!("finding value");

    match op {
        FindOp::MostCommon => {
            println!(" most common");
        },
        FindOp::LeastCommon => {
            println!(" least common");
        }
    }
    */

    loop {
        // println!("column index: {}", column_index);
        let mut on_values: Vec<&Vec<u8>> = Vec::with_capacity(working_list.len());
        let mut off_values: Vec<&Vec<u8>> = Vec::with_capacity(working_list.len());

        for value in working_list {
            /*{
                let mut msg = String::with_capacity(value.len());

                for v in value {
                    msg.write_char((*v as char)).unwrap();
                }

                println!("checking: {} bit: {}", msg, (value[column_index] as char));
            }*/

            if value[column_index] == UNICODE_ONE {
                on_values.push(value);
            } else {
                off_values.push(value);
            }
        }

        if on_values.len() >= off_values.len() {
            match op {
                FindOp::MostCommon => working_list = on_values,
                FindOp::LeastCommon => working_list = off_values
            }
        } else {
            match op {
                FindOp::MostCommon => working_list = off_values,
                FindOp::LeastCommon => working_list = on_values
            }
        }

        if working_list.len() > 1 {
            if column_index == (column_size - 1) {
                return Err("cannot go past last column. failed".to_owned());
            }

            column_index += 1;
        } else if working_list.len() == 1 {
            let first = working_list.first().unwrap();
            return Ok(u8_slice_to_u64(*first));
        } else {
            return Err("empty list. nothing to do".to_owned());
        }
    }
}

fn part_b() -> lib::ResultBoxE<()> {
    let mut input = lib::lines_from_arg_or_default("./day03/input.txt")?.peekable();

    let mut expected: usize = 0;
    let mut working: Vec<Vec<u8>> = Vec::new();

    let start = Instant::now();

    if let Some(line) = input.peek() {
        if let Ok(ln) = line {
            expected = ln.len();
        }
    }

    for line in input {
        if let Ok(ln) = line {
            if ln.len() != expected {
                println!("unexpected line size. all input lines must be the same length. line: {}", ln);
                return Ok(());
            }

            for ch in ln.chars() {
                if !(ch == '0' || ch == '1') {
                    println!("unexpected character given in input. all characters must 1 or 0. line: {}", ln);
                    return Ok(());
                }
            }

            working.push(ln.as_bytes().to_owned());
        } else {
            println!("failed to read line from input");
        }
    }

    if working.is_empty() {
        println!("no data to work with. was input empty?");
        return Ok(());
    }

    let oxygen_value = find_value(expected, &working, FindOp::MostCommon)?;
    let co2_value = find_value(expected, &working, FindOp::LeastCommon)?;

    // println!("oxygen value: {} {:b}\nco2 value: {} {:b}", oxygen_value, oxygen_value, co2_value, co2_value);

    let duration = start.elapsed();
    println!("part b\n{}", oxygen_value * co2_value);
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
