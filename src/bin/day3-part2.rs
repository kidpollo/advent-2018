extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

struct Claim(i32, (i32, i32), (i32, i32));

fn main()-> std::io::Result<()> {
    let mut file = File::open("../../day-3-input")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut claims = Vec::new();
    let mut fabric = vec![vec![0; 1000]; 1000];

    for line in lines.iter() {
        let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        let claim = re.captures(line).unwrap();

        claims.push(Claim(claim.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                          (claim.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                           claim.get(3).unwrap().as_str().parse::<i32>().unwrap()),
                          (claim.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                           claim.get(5).unwrap().as_str().parse::<i32>().unwrap())));

    }

    for claim in claims.iter() {
        let Claim(_elf, (pos_x, pos_y), (width, height)) = claim;
        for claimed_x in *pos_x..(pos_x + width) {
            for claimed_y in *pos_y..(pos_y + height) {
                if let Some(foo) = fabric[claimed_x as usize].get_mut(claimed_y as usize){
                    match foo {
                        0 => *foo = 1,
                        1 => *foo = 2,
                        _ => continue,
                    }
                }
            }
        }
    }

    for claim in claims.iter() {
        let Claim(elf, (pos_x, pos_y), (width, height)) = claim;
        let mut has_overlap = false;

        for claimed_x in *pos_x..(pos_x + width) {
            for claimed_y in *pos_y..(pos_y + height) {
                if let Some(foo) = fabric[claimed_x as usize].get_mut(claimed_y as usize){
                    match foo {
                        2 => has_overlap = true,
                        _ => continue,
                    }
                }
            }
        }

        if !has_overlap {
            println!("Overlap elf: {}", elf);
            break;
        }
    }


    Ok(())
}
