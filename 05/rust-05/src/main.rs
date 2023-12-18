use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

struct AgroKey {
    src: u64,
    length: u64,
    dst: u64,
}

struct AgroMap {
    table: Vec<AgroKey>,
}

impl AgroKey{
    fn new(input: &str) -> AgroKey {
        let parts = input.split(" ").collect::<Vec<_>>();
        AgroKey {
            src: parts[1].parse::<u64>().unwrap(),
            length: parts[2].parse::<u64>().unwrap(),
            dst: parts[0].parse::<u64>().unwrap(),
        }
    }
}

impl AgroMap {

    fn description(&self) {
        for key in &self.table {
            println!("{} {} {}", key.src, key.length, key.dst);
        }
    }

    fn new(input: &str) -> AgroMap {
        // TODO make sure map is sorted
        AgroMap {
            table: input.split("\n").map(|x| AgroKey::new(x)).collect::<Vec<_>>(),
        }
    }

    fn lookup(&self, key: u64) -> u64 {
        for k in &self.table {
            if k.src <= key && key < k.src + k.length {
                return k.dst + (key - k.src);
            }
        }
        return key;
    }
}

fn main() {

    let input = include_str!("../input.txt");

    let seperator = Regex::new(r"([a-z\-]*\s?[a-z]*?:\n?)").expect("Invalid regex");
    let splits: Vec<_> = seperator.split(input)
            .into_iter()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            // .map(|x|
            //     x.split("\n").map(
            //         |y| y.split(" ").map(|&z| z.parse::<u64>().unwrap()).collect::<Vec<_>>()
            //     ).collect::<Vec<_>>()
            // )

        .collect();

    // for split in splits {
    //     dbg!(split);
    //     // println!("\"{}\"", split);
    // }
    let seeds = splits[0].split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut seeds_iter = seeds.iter();
    dbg!(seeds_iter.take(2).collect::<Vec<_>>());
    dbg!(seeds_iter.take(2));


    dbg!(seeds.len());

    // let maps = splits[1..].iter().map(|&x| AgroMap::new(x)).collect::<Vec<_>>();


    // dbg!(maps.len());


    // let min_value = seeds.iter().map(|&x| {
    //     let mut key = x;
    //     for m in &maps {
    //         key = m.lookup(key);
    //     }
    //     key
    // }).min();
    // match min_value {
    //     Some(min) => println!( "Min value: {}", min ),
    //     None      => println!( "Vector is empty" ),
    // }

}