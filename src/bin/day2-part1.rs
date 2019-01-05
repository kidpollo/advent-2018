use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn main()-> std::io::Result<()> {
    let mut file = File::open("../../day-2-input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines_iterator = contents.lines();

    let mut checksum_count = HashMap::new();

    for line in lines_iterator {
        let mut leters = HashMap::new();

        for ch in line.chars() {
            let counter = leters.entry(ch).or_insert(0);
            *counter += 1;
        }

        let mut visited = HashSet::new();

        for (key, val) in leters.iter() {
            if !visited.contains(&val.to_string()) {
                let counter = checksum_count.entry(val.to_string()).or_insert(0);
                *counter += 1;
                visited.insert(val.to_string());
            }
        }
    }

    for (key, val) in checksum_count.iter() {
        println!("key: {} val: {}", key, val);
    }

    Ok(())
}
