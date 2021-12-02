use std::{
    path::{Path, PathBuf}, 
    fs::{File, canonicalize}, 
    io::{BufReader, BufRead, Lines, Result as IoResult}, 
    time::Duration, 
    fmt::Write,
    error
};

pub type ResultBoxE<T> = Result<T, Box<dyn error::Error>>;

pub fn read_file_lines<P>(path: P) -> IoResult<Lines<BufReader<File>>>
where 
    P: AsRef<Path>
{
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    Ok(buf.lines())
}

pub fn lines_from_arg_or_default<P>(path: P) -> IoResult<Lines<BufReader<File>>>
where
    P: AsRef<Path>
{
    let mut file_given = PathBuf::from(path.as_ref());
    let mut args = std::env::args();
    args.next();

    loop {
        let arg = match args.next() {
            Some(a) => a,
            None => break
        };

        file_given = canonicalize(&arg)?;
    }

    read_file_lines(file_given)
}

pub fn first_non_zero_duration(duration: Duration) -> String {
    if duration.as_secs() != 0 {
        format!("{}s", duration.as_secs())
    } else if duration.as_millis() != 0 {
        format!("{}ms", duration.as_millis())
    } else if duration.as_micros() != 0 {
        format!("{}μs", duration.as_micros())
    } else {
        format!("{}ns", duration.as_nanos())
    }
}

pub const SECOND: u128 = 1000000000;
pub const MINUTE: u128 = 60000000000;
pub const HOUR: u128 = 3600000000000;

pub fn format_duration(duration: Duration) -> String {
    let mut rtn = String::new();
    let mut running: u128 = duration.as_nanos();

    rtn.write_fmt(format_args!("{:02}:", running / HOUR)).unwrap();
    running %= HOUR;

    rtn.write_fmt(format_args!("{:02}:", running / MINUTE)).unwrap();
    running %= MINUTE;

    rtn.write_fmt(format_args!("{:02}.", running / SECOND)).unwrap();
    running %= SECOND;

    let ns = duration.as_nanos();

    rtn.write_fmt(format_args!("{:09}\n{}\n{}ns", running, first_non_zero_duration(duration), ns)).unwrap();
    rtn
}

pub fn get_debug_file<P>(dir: P, name: &str) -> IoResult<File>
where
    P: Into<PathBuf>
{
    let mut buf = dir.into();
    buf.push(format!("{}.debug.txt", name));
    File::create(buf)
}
