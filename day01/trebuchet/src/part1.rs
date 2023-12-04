use std::time::Instant;

const RADIX: u32 = 10;

fn main() {
    let now = Instant::now();

    // let contents = include_str!("../test_input");
    let contents = include_str!("../input");

    let mut sum = 0;
    contents.split("\r\n")
        .for_each(|line| {
            let mut digs = line.chars()
                .filter(|c| c.is_digit(RADIX))
                .peekable();
            let val = RADIX * digs.peek().unwrap().to_digit(RADIX).unwrap() 
                + digs.last().unwrap().to_digit(RADIX).unwrap();
            sum += val;
        });

    println!("Answer: {:?}", sum);
    println!("Time: {:?}", now.elapsed());
}