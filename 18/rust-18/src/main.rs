use std::fs;
use std::collections::HashMap;
use nalgebra::RealField;
use phf::phf_map;

#[macro_use]
extern crate queues;

use queues::*;

/** Structs */

#[derive(Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
struct Vector {
    x: i64,
    y: i64,
}

impl Vector {
    const fn new(x: i64, y: i64) -> Vector {
        Vector {x, y}
    }
}


/** Statics */

static UNIT_VECTORS: phf::Map<&'static str, Vector> = phf_map! {
    "U" => Vector::new(0, 1),
    "D" => Vector::new(0, -1),
    "L" => Vector::new(-1, 0),
    "R" => Vector::new(1, 0),
};

static DIRECTIONS: phf::Map<&'static str, Vector> = phf_map! {
    "U" => Vector::new(0, 1),
    "D" => Vector::new(0, -1),
    "L" => Vector::new(-1, 0),
    "R" => Vector::new(1, 0),
    "UR" => Vector::new(1, 1),
    "UL" => Vector::new(-1, 1),
    "DR" => Vector::new(1, -1),
    "DL" => Vector::new(-1, -1),
};

/** Integer math functions */
fn dot(v1: &Vector, v2: &Vector) -> i64 {
    v1.x * v2.x + v1.y * v2.y
}

fn mul(v1: &Vector, s: i64) -> Vector {
    Vector::new(v1.x * s, v1.y * s)
}

fn add(p : &Point, v: &Vector) -> Point {
    Point{x: p.x + v.x, y: p.y + v.y}
}

/** Functions */

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

fn parts_to_instructions(parts: &Vec<String>) -> (String, i64) {
    (parts[0].clone(), parts[1].parse::<i64>().unwrap())
}

fn parts_to_instructions_2(parts: &Vec<String>) -> (String, i64) {
    let direction = match &parts[2][7..8] {
        "0" => "R",
        "1" => "D",
        "2" => "L",
        "3" => "U",
        _ => panic!("Unknown direction"),
    };
    let length = i64::from_str_radix(&parts[2][2..7], 16).expect("Unable to parse length as i64");

    (String::from(direction), length)
}


fn instructions_to_vector(direction: &String, length: i64) -> Vector{
    let direction = UNIT_VECTORS.get(direction).unwrap();

    mul(direction, length)
}

fn print_polygon(polygon: &Vec<Point>) {
    let min_x = polygon.iter().min_by_key(|p| p.x).unwrap().x;
    let max_x = polygon.iter().max_by_key(|p| p.x).unwrap().x;
    let min_y = polygon.iter().min_by_key(|p| p.y).unwrap().y;
    let max_y = polygon.iter().max_by_key(|p| p.y).unwrap().y;

    println!("min_x: {}, max_x: {}, min_y: {}, max_y: {}", min_x, max_x, min_y, max_y);

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if polygon.contains(&Point{x, y}) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn print_canvas(canvas: &Vec<Vec<char>>) {
    for row in canvas.iter().rev() {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
}

fn polygon_limits(polygon: &Vec<Point>) -> (Point, Point) {
    let min_x = polygon.iter().min_by_key(|p| p.x).unwrap().x;
    let max_x = polygon.iter().max_by_key(|p| p.x).unwrap().x;
    let min_y = polygon.iter().min_by_key(|p| p.y).unwrap().y;
    let max_y = polygon.iter().max_by_key(|p| p.y).unwrap().y;

    (Point{x: min_x, y: min_y}, Point{x: max_x, y: max_y})
}

fn fill_polygon_interior(canvas: &mut Vec<Vec<char>>, start: &Point){

    let mut queue: Queue<Point> = queue![];
    let _ = queue.add(start.clone());

    let canvas_max_x = canvas[0].len() as i64;
    let canvas_max_y = canvas.len() as i64;

    canvas[start.y as usize][start.x as usize] = 'O';
    while let Ok(candidate) = queue.remove() {

        canvas[candidate.y as usize][candidate.x as usize] = '#';

        // Add surrounding tiles
        for d in DIRECTIONS.values() {
            let p = add(&candidate, d);

            // Check bounds
            if p.x < 0 || p.y < 0 {
                continue;
            }
            if p.x >= canvas_max_x || p.y >= canvas_max_y {
                continue;
            }

            if canvas[p.y as usize][p.x as usize] == '.' {
                canvas[p.y as usize][p.x as usize] = '#';
                let _ = queue.add(p);
            }
    
        }
    }



}


fn find_any_polygon_interior_point(canvas: & Vec<Vec<char>>) -> Point{
    for y in 1..(canvas.len()-1) {
        for x in 0..canvas[y].len() {
            if canvas[y][x] == '#' && canvas[y-1][x] == '#' && canvas[y+1][x] == '#' {
                return Point{x: (x+1) as i64, y: y as i64};
            }
        }
    }
    panic!("No interior point found");
}

/** Main */

fn part1(){

    let input = parse_words("input.txt");

    // Build list of instructions
    let instructions = input.iter()
        .map(|s| parts_to_instructions(s))
        .collect::<Vec<_>>();

    let vectors = instructions.iter()
    .map(|i| instructions_to_vector(&i.0, i.1))
    .collect::<Vec<_>>();

    // Build polygon
    let polygon = vectors.iter()
        .scan(Point{x: 0, y: 0}, |state, v| {
            state.x += v.x;
            state.y += v.y;
            Some(Point{x: state.x, y: state.y})
        })
        .collect::<Vec<_>>();

    // Fill out canvas
    let (min, max) = polygon_limits(&polygon);
    // println!("min: {:?}, max: {:?}", min, max);
    let mut canvas = vec![vec!['.'; (max.x - min.x + 1) as usize]; (max.y - min.y + 1) as usize];
    // println!("canvas size: {}x{}", canvas[0].len(), canvas.len());

    // Draw polygon
    let mut coords = Point{x: -min.x, y: -min.y};
    for instruction in &instructions {
        let direction = &instruction.0;
        let magnitude = instruction.1;
        
        for i in 0..magnitude {
            match direction.as_str() {
                "U" => coords.y += 1,
                "D" => coords.y -= 1,
                "L" => coords.x -= 1,
                "R" => coords.x += 1,
                _ => panic!("Unknown direction"),
            }
            // println!("coords: {:?}", coords);
            canvas[coords.y as usize][coords.x as usize] = '#';
        }
    }

    let polygon_length = instructions.iter().fold(0, |acc, i| acc + i.1);

    // Homebrew flood fill algorithm
    let interior_point = find_any_polygon_interior_point(& canvas);
    fill_polygon_interior(&mut canvas, &interior_point);

    // print_canvas(&canvas);

    // Count size of lagoon
    let area = canvas.iter().fold(0, |acc, row| acc + row.iter().filter(|c| **c == '#').count());

    println!("Part 1 area: {}", area);
    
}


// Implementation #1 of polymino area calculation
fn polyomino_area_from_instructions(instructions: &Vec<(String, i64)>) -> i64 {

    let mut point = Point{x: 0, y: 0};
    let mut area: i64 = 0;

    for (direction, distance) in instructions{
        let direction_vector = DIRECTIONS.get(&direction).unwrap();
        let relative_movement = mul(&direction_vector, *distance);
        let x_n = add(&point, &relative_movement);
        area += x_n.x * point.y - point.x * x_n.y;

        point = x_n;
    }

    area / 2
}

// Implementation #2 of polymino area calculation
fn polyomino_area_from_instructions_2(instructions: &Vec<(String, i64)>) -> i64 {

    let mut area = 0;
    let mut x = 0;
    let mut y = 0;

    for (direction, distance) in instructions{
        match direction.as_str() {
            "U" => {
                y += distance;
            },
            "D" => {
                y -= distance;
            },
            "L" => {
                x -= distance;
                area -= y * distance;
            },
            "R" => {
                x += distance;
                area += y * distance;
            },
            _ => panic!("Unknown direction"),
        }
    }

    area
}


fn part2(){
    let input: Vec<Vec<String>> = parse_words("input.txt");

    // Build list of instructions
    let instructions = input.iter()
        .map(|s| parts_to_instructions_2(s))
        .collect::<Vec<_>>();
    
    let vectors = instructions.iter()
        .map(|i| instructions_to_vector(&i.0, i.1))
        .collect::<Vec<_>>();

    // Build polygon
    let polygon = vectors.iter()
        .scan(Point{x: 0, y: 0}, |state, v| {
            state.x += v.x;
            state.y += v.y;
            Some(Point{x: state.x, y: state.y})
        })
        .collect::<Vec<_>>();

    let polygon_length = instructions.iter().fold(0, |acc, i| acc + i.1);

    let area = polyomino_area_from_instructions(&instructions) + polygon_length / 2 + 1;
    println!("Part 2 area: {}", area);
}



fn main() {
    part1();
    part2();
}
