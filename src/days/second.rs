use std::{
    fs::File,
    io::{Read, Result},
};

pub fn solution() -> Result<()> {
    let mut file = File::open("inputs/2.txt")?;
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let lines = contents.lines();
    let mut total_first = 0;
    let mut total_second = 0;
    lines.for_each(|line| {
        let mut increasing = 0;
        let mut decreasing = 0;
        let mut differ = 0;
        let mut iter = line.split_whitespace().peekable();
        while iter.peek() != None {
            // this is very unsafe but in this specific case it cannot panic unless the file is
            // wrong
            let current: i32 = iter
                .next()
                .unwrap()
                .parse()
                .expect("The input file is wrong!");
            if iter.peek() == None {
                break;
            }
            let next: i32 = iter.peek().unwrap().parse().unwrap();
            if current < next {
                increasing += 1
            }
            if current > next {
                decreasing += 1
            }
            let difference = (current - next).abs();
            if 1 <= difference && difference <= 3 {
                differ += 1
            }
        }
        if (increasing == 5 || decreasing == 5) && differ == 5 {
            total_first += 1;
        }
        if (increasing > 3 || decreasing > 3) && differ > 3 {
            total_second += 1;
        }
    });

    println!("The total sum of safe reports is: {}", total_first);
    println!(
        "The total sum of dampened safe reports is: {}",
        total_second
    );

    Ok(())
}
