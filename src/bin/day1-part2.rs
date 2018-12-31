use std::fs::File;
use std::collections::HashSet;
use std::io::prelude::*;

fn main()-> std::io::Result<()> {
    let mut file = File::open("../../day-1-input")?;
    let mut contents = String::new();
    let mut known_frequencies: HashSet<i32> = HashSet::new();
    let mut acc: i32 = 0;
    file.read_to_string(&mut contents)?;
    //contents.clear();
    //contents.push_str("1\n-1");
    //contents.push_str("+7\n+7\n-2\n-7\n-4");
    contents.lines().cycle()
        .find(|line|
              is_in_known_frequencies(&mut acc, line.parse::<i32>().unwrap(), &mut known_frequencies)
        );
    println!("Result: {:?}", acc);
    Ok(())
}

fn is_in_known_frequencies(acc: &mut i32, freq: i32, known_frequencies: &mut HashSet<i32>) -> bool {
    if !known_frequencies.insert(*acc) {
        println!("{}, found!", *acc);
        true
    } else {
        *acc = *acc + freq;
        false
    }
}
