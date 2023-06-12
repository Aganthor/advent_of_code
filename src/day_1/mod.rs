pub mod day_1_solution {
    use std::fs::File;
    use std::io::{BufRead, BufReader};


    //
    //Day 1 advent of code solver!
    //
    pub fn solve_day_1() -> u32 {
        let file = File::open("./input_day1.txt").expect("Failed to read input file.");
        let file = BufReader::new(file);
        let mut depth_data : Vec<u32> = Vec::new();

        for line in file.lines() {
            depth_data.push(line.unwrap().trim().parse::<u32>().unwrap());
        }
        let mut next_iter = depth_data.iter().peekable();
        let mut count: u32 = 0;
        for (_current, &current_reading) in depth_data.iter().enumerate() {
            next_iter.next();
            if let Some(&&next_reading) = next_iter.peek() {
                if next_reading > current_reading {
                    count += 1;
                }
            }
        }
        count
    }
}