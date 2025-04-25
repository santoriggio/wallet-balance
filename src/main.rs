use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

fn main() -> Result<(), usize> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let f = match File::open(path) {
        Ok(f) => f,
        Err(err) => {
            eprintln!("Error while opening file: {}", err);
            return Err(0);
        }
    };

    let reader = BufReader::new(f);

    let start = Instant::now();
    let mut total = 0.0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                total += get_value_from_line(line);
            }
            Err(err) => eprintln!("Error while read the line {}", err),
        }
    }

    let end = start.elapsed();

    println!("Total is {} in {:?}", total, end);

    return Ok(());
}

const AMOUNT_IDX: usize = 5;

fn get_value_from_line(line: String) -> f32 {
    let splitted: Vec<&str> = line.split(',').collect();
    return splitted[AMOUNT_IDX].parse().unwrap_or(0.0);
}
