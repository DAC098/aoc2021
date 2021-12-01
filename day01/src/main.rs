use std::time::Instant;

fn part_a() -> () {
    let input_result = lib::lines_from_arg_or_default("./day01/input.txt");

    if input_result.is_err() {
        println!("{:?}", input_result.unwrap_err());
        return;
    }

    let mut prev_depth: i32 = 0;
    let mut increase_count: u32 = 0;

    let start = Instant::now();

    for line in input_result.unwrap() {
        if let Ok(ln) = line {
            if let Ok(depth) = ln.parse::<i32>() {
                if prev_depth != 0 && depth > prev_depth {
                    increase_count += 1;
                }

                prev_depth = depth;
            } else {
                println!("failed to parse line as i32. line: {}", ln);
                return;
            }
        } else {
            println!("error when retrieving line of file");
            return;
        }
    }

    let duration = start.elapsed();
    println!("part a\n{}", increase_count);
    println!("{}", lib::format_duration(duration));
}

fn part_b() -> () {
    let input_result = lib::lines_from_arg_or_default("./day01/input.txt");

    if input_result.is_err() {
        println!("{:?}", input_result.unwrap_err());
        return;
    }

    let mut prev_window: i32 = 0;
    let mut window_increase_count: u32 = 0;
    let mut window: [i32; 3] = [0, 0, 0];
    let mut window_index: usize = 0;
    let mut total_measurements: u32 = 0;

    let start = Instant::now();

    for line in input_result.unwrap() {
        if let Ok(ln) = line {
            if let Ok(depth) = ln.parse::<i32>() {
                window[window_index] = depth;
                window_index = (window_index + 1) % 3;

                total_measurements += 1;

                if total_measurements >= 3 {
                    let window_depth: i32 = window.iter().sum();

                    if prev_window != 0 && window_depth > prev_window {
                        window_increase_count += 1;
                    }

                    prev_window = window_depth;
                }
            } else {
                println!("failed to parse line as i32. line: {}", ln);
                return;
            }
        } else {
            println!("error when retrieving line of file");
            return;
        }
    }

    let duration = start.elapsed();
    println!("part b\n{}", window_increase_count);
    println!("{}", lib::format_duration(duration));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    part_a();
    part_b();
    Ok(())
}
