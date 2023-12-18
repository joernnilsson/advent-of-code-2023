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
    // fn get_dst(&self, key: u64) -> Option<u64> {
    //     if self.src <= key && key < self.src + self.length {
    //         return Some(self.dst + (key - self.src));
    //     }
    // }

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

    let maps = splits[1..].iter().map(|&x| AgroMap::new(x)).collect::<Vec<_>>();


    dbg!(maps.len());
    // let seed_to_soil = AgroMap::new(splits[1]);
    // let soil_to_fertilizer = AgroMap::new(splits[2]);

    // for s in seeds {
    //     let mut key = s;
    //     for m in &maps {
    //         key = m.lookup(key);
    //     }
    //     println!("{}", key);
    // }

    let min_value = seeds.iter().map(|&x| {
        let mut key = x;
        for m in &maps {
            key = m.lookup(key);
        }
        key
    }).min();
    match min_value {
        Some(min) => println!( "Min value: {}", min ),
        None      => println!( "Vector is empty" ),
    }

}

// fn main2() {
//     if let Ok(mut lines) = read_lines("./input.txt") {

//         let first = lines.next().unwrap().unwrap();
//         let first_parts: Vec<&str> = first.split(" ").collect();
//         let seeds_str = &first_parts[1..];
//         let seeds: Vec<u64> = seeds_str.iter().map(|&x| x.parse::<u64>().unwrap()).collect();

//         dbg!(seeds);


//         for p in lines{

//             println!("{}", p.unwrap());
//         }



//         // println!("{}", first_parts);
//         // println!("{}", lines.next()?);
//         // println!("{}", lines.next()?);

//         // Consumes the iterator, returns an (Optional) String
//         // for line in lines {
//         //     if let Ok(ip) = line {
//         //         println!("{}", ip);
//         //     }
//         // }



//     }
// }

// // The output is wrapped in a Result to allow matching on errors
// // Returns an Iterator to the Reader of the lines of the file.
// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where P: AsRef<Path>, {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }