use std::{
    fs::File,
    io::{Read, Result},
};

pub fn solution() -> Result<()> {
    let mut file = File::open("inputs/11.txt")?;
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let lines = contents.lines();
    let mut left: [i32; 1000] = [0; 1000];
    let mut right: [i32; 1000] = [0; 1000];
    lines.enumerate().for_each(|(i, line)| {
        let str_one = &line[0..5];
        let str_two = &line[8..13];

        left[i] = str_one.parse().expect("Input file is wrong!");
        right[i] = str_two.parse().expect("Input file is wrong!");
    });

    // Part 1
    left.sort();
    right.sort();

    let mut total = 0;

    for it in left.iter().zip(right.iter()) {
        let (l, r) = it;
        total += (*l - *r).abs();
    }

    println!("The sum of the sorted differences is: {}", total);

    // Part 2
    let mut total = 0;
    left.iter().for_each(|it| {
        let mut count = 0;
        right.iter().for_each(|it2| {
            if *it == *it2 {
                count += 1;
            }
        });
        total += *it * count;
    });

    println!("The sum of all the similarity scores is: {}", total);

    Ok(())
}
