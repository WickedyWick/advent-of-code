use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let simple = false;

    let mut path = "input1.txt";
    if simple {
        path = "inputSimple1.txt";
    }

    let file = match File::open(path) {
        Ok(f) => f,
        Err(_err) => panic!("Err opening file. Err: {}", _err.to_string())
    };

    let reader = BufReader::new(file);
    let mut h_pos: i32 = 0;
    let mut depth: i32 = 0;

    for line in  reader.lines() {
        if let split_lines = line.unwrap().split(' ').collect::<Vec<&str>>() {
            _ = match split_lines[0] {
                "forward" => {
                    h_pos += split_lines[1].parse::<i32>().unwrap();
                },
                "down" => {
                    depth += split_lines[1].parse::<i32>().unwrap();
                },
                "up" => {
                    depth -= split_lines[1].parse::<i32>().unwrap();
                },
                _ => continue
            }
        }

        /*
        let split_lines = match line {
            Ok(l) => l.split().collect::<Vec<&str>>(),
            Err(_err) => panic!("Err reading line. Err: {}", _err.to_string())
        };*/

        //println!("{}", val);

    }
    println!("RESULT: {}", depth * h_pos);
}
