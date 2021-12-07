use std::{collections::HashMap, ops::RangeInclusive, time::Instant};

struct Coord (u16, u16);

fn parse_coord(section: &str) -> lib::error::Result<Coord> {
    if let Some((x_str,y_str)) = section.split_once(',') {
        let x: u16;
        let y: u16;

        if let Ok(parse) = x_str.parse() {
            x = parse;
        } else {
            return Err(format!("failed to parse x coord as an unsigned integer. given: {}", x_str).into());
        }

        if let Ok(parse) = y_str.parse() {
            y = parse;
        } else {
            return Err(format!("failed to parse y coord as an unsigned integer. given: {}", y_str).into());
        }

        Ok(Coord(x,y))
    } else {
        Err(format!("invalid format given. argument must be two numbers split by a \",\". given: {}", section).into())
    }
}

fn parse_line(line: &str) -> lib::error::Result<(Coord, Coord)> {
    let mut split = line.split_whitespace();
    let first: Coord;
    let second: Coord;

    if let Some(first_str) = split.next() {
        first = parse_coord(first_str).map_err(
            |e| lib::error::Error::General(format!("first coordinate failed:\n{}\nline: {}", e, line))
        )?;
    } else {
        return Err(format!("unexpected end of first coordinate").into());
    }

    if split.next().is_none() {
        return Err(format!("unexpected end of direction delimiter").into())
    }

    if let Some(second_str) = split.next() {
        second = parse_coord(second_str).map_err(
            |e| lib::error::Error::General(format!("second coordinate failed:\n{}\nline: {}", e, line))
        )?;
    } else {
        return Err(format!("unexpected end of second coordinate").into())
    }

    Ok((first, second))
}

fn insert_grid_point(grid: &mut HashMap<u16, HashMap<u16, u16>>, x: &u16, y: &u16) -> bool {
    if let Some(y_grid) = grid.get_mut(x) {
        if let Some(count) = y_grid.get_mut(y) {
            *count += 1;
            return true;
        } else {
            y_grid.insert(*y, 1);
        }
    } else {
        let mut y_grid = HashMap::new();
        y_grid.insert(*y, 1);
        grid.insert(*x, y_grid);
    }

    return false;
}

fn get_y_range(first: &Coord, second: &Coord) -> RangeInclusive<u16> {
    if first.1 < second.1 {
        first.1..=second.1
    } else {
        second.1..=first.1
    }
}

fn get_x_range(first: &Coord, second: &Coord) -> RangeInclusive<u16> {
    if first.0 < second.0 {
        first.0..=second.0
    } else {
        second.0..=first.0
    }
}

fn part_a() -> lib::error::Result<()> {
    let input = lib::lines_from_arg_or_default("./day05/input.txt")?;

    let mut known_grid: HashMap<u16, HashMap<u16, u16>> = HashMap::new();
    let mut intersecting: u64 = 0;

    let start = Instant::now();

    for line in input {
        if let Ok(ln) = line {
            let (first, second) = parse_line(ln.as_str())?;

            if first.0 == second.0 {
                // increment y
                for pos in get_y_range(&first, &second) {
                    if insert_grid_point(&mut known_grid, &first.0, &pos) {
                        intersecting += 1;
                    }
                }
            } else if first.1 == second.1 {
                // increment x
                for pos in get_x_range(&first, &second) {
                    if insert_grid_point(&mut known_grid, &pos, &first.1) {
                        intersecting += 1;
                    }
                }
            } else if first.0 == second.1 && first.1 == second.0 {
                // diagonal
            }
        }
    }

    let duration = start.elapsed();
    println!("part a\n{}", intersecting);
    lib::print_duration(&duration);

    Ok(())
}

fn main() {
    if let Err(e) = part_a() {
        println!("{}", e);
    }
}
