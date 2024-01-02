use std::fs;
use itertools::Itertools;

extern crate nalgebra as na;
use na::{U2, SMatrix, Vector3, Vector2};

/** Structs */

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[derive(Debug)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    const fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector {x, y, z}
    }
}


#[derive(Debug)]
struct Hailstone {
    p: Point,
    v: Vector,
}

/** Functions */

fn words_to_hailstone(words: Vec<String>) -> Hailstone {
    let p = Point {
        x: words[0].parse::<f64>().unwrap(),
        y: words[1].parse::<f64>().unwrap(),
        z: words[2].parse::<f64>().unwrap(),
    };
    let v = Vector {
        x: words[4].parse::<f64>().unwrap(),
        y: words[5].parse::<f64>().unwrap(),
        z: words[6].parse::<f64>().unwrap(),
    };
    Hailstone {
        p,
        v,
    }
}


fn parse_words(filename: &str) -> Vec<Vec<String>>{
    let lines: Vec<Vec<String>> = fs::read_to_string(filename)
        .expect("Unable to read file")
        .split("\n")
        .map(|line| line.trim())
        .map(|line| line.split(" ")
            .map(|sl| String::from(sl))
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();
    lines
}

// Solve system of 2 equations directly
fn hailstones_intersects_witin(h1: &Hailstone, h2: &Hailstone, min: f64, max: f64) -> bool {

    println!("Hailstone A: {}, {}, {} @ {}, {}, {}", h1.p.x, h1.p.y, h1.p.z, h1.v.x, h1.v.y, h1.v.z);
    println!("Hailstone B: {}, {}, {} @ {}, {}, {}", h2.p.x, h2.p.y, h2.p.z, h2.v.x, h2.v.y, h2.v.z);

    let x1 = h1.p.x;
    let y1 = h1.p.y;
    
    let x2 = h1.p.x + h1.v.x;
    let y2 = h1.p.y + h1.v.y;

    let x3 = h2.p.x;
    let y3 = h2.p.y;

    let x4 = h2.p.x + h2.v.x;
    let y4 = h2.p.y + h2.v.y;


    let x_den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    if x_den == 0.0 {
        println!("Hailstones' paths are parallel; they never intersect.");
        return false;
    }

    let x_nom = (x1*y2 - y1*x2) * (x3 - x4) - (x1 - x2) * (x3*y4 - y3*x4);

    let y_nom = (x1*y2 - y1*x2) * (y3 - y4) - (y1 - y2) * (x3*y4 - y3*x4);
    let y_den = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    let x = x_nom / x_den;
    let y = y_nom / y_den;
    println!("Hailstones' paths will cross inside the test area (at x={}, y={})\n", x, y);

    x > min && x < max && y > min && y < max
}



type Matrix2x2f = SMatrix<f64, 2, 2>;

// Solve system of 2 equations using LU decomposition
fn hailstones_intersects_witin_2(h1: &Hailstone, h2: &Hailstone, min: f64, max: f64) -> bool {

    println!("Hailstone A: {}, {}, {} @ {}, {}, {}", h1.p.x, h1.p.y, h1.p.z, h1.v.x, h1.v.y, h1.v.z);
    println!("Hailstone B: {}, {}, {} @ {}, {}, {}", h2.p.x, h2.p.y, h2.p.z, h2.v.x, h2.v.y, h2.v.z);

    // parametric form 

    // 1: p1.x + v1.x * t1 = p2.x + v2.x * t2
    // 2: p1.y + v1.y * t1 = p2.y + v2.y * t2

    // 1: t1*v1.x - t2*v2.x = p2.x - p1.x
    // 2: t1*v1.y - t2*v2.y = p2.y - p1.y

    let a = Matrix2x2f::new( 
        h1.v.x , -h2.v.x,
        h1.v.y, -h2.v.y
    );

    let b = Vector2::new(
        h2.p.x - h1.p.x,
        h2.p.y - h1.p.y,
    );

    let decomp = a.lu();
    let t = decomp.solve(&b);

    // return
    return if let Some(t) = t {


        let x = h1.p.x + h1.v.x * t[0];
        let y = h1.p.y + h1.v.y * t[0];

        println!("Hailstones' paths will cross (at x={}, y={})", x, y);
        println!("  (at t1={}, t2={})", t[0], t[1]);

        if t[0] < 0.0 {
            println!("Hailstones A intersects in the past");
            return false;
        }
        if t[1] < 0.0 {
            println!("Hailstones B intersects in the past");
            return false;
        }

        let valid = x > min && x < max && y > min && y < max;
        println!("  inside: {}", valid);

        println!("");
        valid
    } else  {
        false
    }
}


fn part1() {

    let filename = "input.txt";
    let test_area_min: f64 = 200000000000000.0;
    let test_area_max: f64 = 400000000000000.0;

    // let filename = "example.txt";
    // let test_area_min: f64 = 7.0;
    // let test_area_max: f64 = 27.0;

    let lines = parse_words(filename);
    let stripped_lines = lines.iter()
        .map(|words| words.iter()
            .map(|word| word.to_string().replace(",", ""))
            .filter(|word| word.len() > 0)
            .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

    let hailstones: Vec<Hailstone> = stripped_lines.iter()
        .map(|words| words_to_hailstone(words.to_vec()))
        .collect::<Vec<_>>();

    // dbg!(&hailstones);

    let intersctions = hailstones.iter().combinations(2)
        .map(|vpair| hailstones_intersects_witin_2(vpair.first().unwrap(), vpair.last().unwrap(), test_area_min, test_area_max))
        .filter(|x| *x)
        .count();

    println!("Part 1 intersections: {}", intersctions);
}


fn main() {
    part1();
}
