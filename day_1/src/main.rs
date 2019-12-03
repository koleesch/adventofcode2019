use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum:i64 = 0;
    let mut count:i64 = 0;
    let lines = read_lines("input.txt").expect("file not found");
      
    for line in lines {
        if let Ok(val) = line {
            
            if let Ok(value) = val.parse::<i64>(){
                
                let mut part = day_1::calculate_single_fuel(value);
                while part > 0   {
                    sum += part;
                    part = day_1::calculate_single_fuel(part);                    
                }                
            }
        }
        count+=1;
    }

    println!("count: {}", count);
    println!("sum: {}", sum);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}