use std::{fmt::{Display, Formatter, Result as FmtResult}, time::{Instant, Duration}, collections::HashSet};

type BingoNum = u16;

#[derive(Clone)]
struct BingoBoard (
    [[(BingoNum, bool); 5]; 5]
);

enum BingoFound {
    Row(usize),
    Column(usize)
}

impl BingoBoard {

    fn check_bingo(&self, column: &usize, row: &usize) -> Option<BingoFound> {
        let mut found_false = false;
        // check column first
        for index in 0..5 {
            if !self.0[*column][index].1 {
                found_false = true;
                break;
            }
        }

        if !found_false {
            return Some(BingoFound::Column(*column));
        }

        found_false = false;
        // check row
        for index in 0..5 {
            if !self.0[index][*row].1 {
                found_false = true;
                break;
            }
        }

        if !found_false {
            Some(BingoFound::Row(*row))
        } else {
            None
        }
    }

    fn set_value(&mut self, v: &BingoNum) -> Option<BingoFound> {
        for column_index in 0..5 {
            for row_index in 0..5 {
                if self.0[column_index][row_index].0 == *v {
                    self.0[column_index][row_index].1 = true;

                    return self.check_bingo(&column_index, &row_index);
                }
            }
        }

        None
    }

    #[allow(dead_code)]
    fn sum_bingo(&self, found: BingoFound) -> u32 {
        match found {
            BingoFound::Column(col) => {
                let mut rtn: u32 = 0;

                for row in 0..5 {
                    rtn += self.0[col][row].0 as u32;
                }

                rtn
            },
            BingoFound::Row(row) => {
                let mut rtn: u32 = 0;

                for col in 0..5 {
                    rtn += self.0[col][row].0 as u32;
                }

                rtn
            }
        }
    }

    fn sum_all_unmarked(&self) -> u32 {
        let mut rtn: u32 = 0;

        for col in 0..5 {
            for row in 0..5 {
                if !self.0[col][row].1 {
                    rtn += self.0[col][row].0 as u32;
                }
            }
        }

        rtn
    }
}

impl Display for BingoBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        for row_index in 0..5 {
            for col_index in 0..5 {
                if col_index == 0 {
                    write!(f, "{:02}", self.0[col_index][row_index].0)?;
                } else {
                    write!(f, " {:02}", self.0[col_index][row_index].0)?;
                }
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn part_a() -> lib::Result<()> {
    let mut input = lib::lines_from_arg_or_default("./day04/input.txt")?;
    
    let mut number_list: Vec<BingoNum> = Vec::new();

    let start = Instant::now();

    if let Some(number_str) = input.next() {
        if let Ok(line) = number_str {
            for num in line.split(',') {
                if let Ok(parse) = num.parse::<BingoNum>() {
                    number_list.push(parse);
                } else {
                    return Err(format!("failed to parse string to unsigned integer. given: {}", num).into());
                }
            }
        } else {
            return Err("failed to read line for number list".into());
        }
    } else {
        return Err("empty file given".into());
    }

    if input.next().is_none() {
        return Err("unexpected end of input".into());
    }

    let mut boards: Vec<BingoBoard> = Vec::new();

    loop {
        let mut board: [[(BingoNum, bool); 5]; 5] = [[(0, false); 5]; 5];

        for row in 0..5 {
            if let Some(line) = input.next() {
                if let Ok(ln) = line {
                    let mut col: usize = 0;

                    for num in ln.split_whitespace() {
                        if col == 5 {
                            return Err(format!("given row has too many numbers. given: {}", ln).into());
                        }

                        if let Ok(parse) = num.parse::<BingoNum>() {
                            board[col][row].0 = parse;
                            col += 1;
                        } else {
                            return Err(format!("failed to parse number to unsigned integer. num: {} line: {}", num, ln).into());
                        }
                    }

                    if col < 5 {
                        return Err(format!("given row has too few numbers. given: {}", ln).into());
                    }
                } else {
                    return Err("failed to read line from input".into());
                }
            } else {
                return Err("unexpected end of input. make sure all boards have 5 columns and 5 rows of numbers".into());
            }
        }

        boards.push(BingoBoard(board));

        if input.next().is_none() {
            break;
        }
    }

    let mut first_winner: Option<(BingoBoard, BingoNum, Duration)> = None;
    let mut last_winner: Option<(BingoBoard, BingoNum)> = None;
    let mut skip_boards: HashSet<usize> = HashSet::new();

    // play bingo
    for called_num in number_list {
        for board in 0..boards.len() {
            if skip_boards.contains(&board) {
                continue;
            }

            if let Some(_win) = boards[board].set_value(&called_num) {
                if first_winner.is_none() {
                    first_winner = Some((boards[board].clone(), called_num, start.elapsed()));
                }

                last_winner = Some((boards[board].clone(), called_num));
                skip_boards.insert(board);
            }
        }
    }

    if let Some(winner) = first_winner {
        let duration = start.elapsed();

        println!("part a\n{}", winner.0.sum_all_unmarked() * (winner.1 as u32));
        lib::print_duration(&winner.2);

        let last_winner = last_winner.unwrap();

        println!("part b\n{}", last_winner.0.sum_all_unmarked() * (last_winner.1 as u32));
        lib::print_duration(&duration);
    } else {
        println!("no winner found?");
    }

    Ok(())
}

fn main() {
    if let Err(e) = part_a() {
        println!("{}", e);
    }
}
