use std::{error::Error, collections::HashMap};
use std::fs;
use std::cmp;

struct Tile {
    tile_type: char,
    beams: HashMap<Direction, bool>,
    energized: bool,
}

impl Tile {
    fn new(tile_type: char) -> Tile {
        Tile {
            tile_type: tile_type,
            beams: HashMap::from([
                (Direction::LEFT, false),
                (Direction::RIGHT, false),
                (Direction::UP, false),
                (Direction::DOWN, false)]
            ),
            energized: false,
        }
    }

    fn heading_is_set(&self, heading: &Heading) -> bool {
        match heading {
            Heading::HORIZONTAL => self.beams[&Direction::LEFT] || self.beams[&Direction::RIGHT],
            Heading::VERTICAL => self.beams[&Direction::UP] || self.beams[&Direction::DOWN],
        }
    }
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Heading {
    HORIZONTAL,
    VERTICAL,
}

fn direction_to_heading(direction: &Direction) -> Heading {
    match direction {
        Direction::LEFT | Direction::RIGHT => Heading::HORIZONTAL,
        Direction::UP | Direction::DOWN => Heading::VERTICAL,
    }
}

fn print_tile(tile: &Tile) {
    print!("{}", tile.tile_type);
}

fn print_tiles(tiles: & Vec<Vec<Tile>>) {
    for line in tiles {
        for tile in line {
            let symbol = match tile.tile_type {
                '.' => if tile.beams[&Direction::LEFT] {
                    '<'
                } else if tile.beams[&Direction::RIGHT] {
                    '>'
                } else if tile.beams[&Direction::UP] {
                    '^'
                } else if tile.beams[&Direction::DOWN] {
                    'v'
                } else {
                    tile.tile_type
                },
                _ => tile.tile_type,
            };
            print!("{}", symbol);
        }
        println!();
    }
}

fn print_energized_tiles(tiles: & Vec<Vec<Tile>>) {
    for line in tiles {
        for tile in line {
            let symbol = match tile.energized {
                true => '#',
                false => '.',
            };
            print!("{}", symbol);
        }
        println!();
    }
}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// Return the next set of coords if they are in bounds
fn next_tile(tiles: &mut Vec<Vec<Tile>>, x: usize, y: usize, direction: &Direction) ->  Result<(usize, usize), Box<dyn std::error::Error>>{
    match direction {
        Direction::LEFT =>
            if x == 0 {
                Err("Out of bounds".into())
            } else {
                Ok((x-1, y))
            },
        Direction::RIGHT =>
            if x == tiles[0].len() - 1 {
                Err("Out of bounds".into())
            } else {
                Ok((x+1, y))
            },
        Direction::UP =>
            if y == 0 {
                Err("Out of bounds".into())
            } else {
                Ok((x, y-1))
            },
        Direction::DOWN =>
            if y == tiles.len() - 1 {
                Err("Out of bounds".into())
            } else {
                Ok((x, y+1))
            }
    }
}

// Walks along a beam/ray and splits into recursive calls if a horizontal or vertcal split is encountered
fn beam_walk(lines: &mut Vec<Vec<Tile>>, x: usize, y: usize, direction_input: Direction) -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let mut x = x;
    let mut y = y;
    let mut direction = direction_input;

    loop {
        let mut tile = &mut lines[y][x];

        // Detetct if we are in a loop
        let heading = direction_to_heading(&direction);
        if tile.heading_is_set(&heading) {
            break;
        }

        // print_tile(tile);
        tile.energized = true;
        match tile.tile_type {
            '.' => {
                tile.beams.insert(direction, true);
                (x, y) = next_tile(lines, x, y, &direction)?;
                // Continue loop if we can

            },
            '/' => {
                match direction {
                    Direction::LEFT => {
                        direction = Direction::DOWN;
                    },
                    Direction::RIGHT => {
                        direction = Direction::UP;
                    },
                    Direction::UP => {
                        direction = Direction::RIGHT;
                    },
                    Direction::DOWN => {
                        direction = Direction::LEFT;
                    },
                }
                (x, y) = next_tile(lines, x, y, &direction)?;
            }
            '\\' => {
                match direction {
                    Direction::LEFT => {
                        direction = Direction::UP;
                    },
                    Direction::RIGHT => {
                        direction = Direction::DOWN;
                    },
                    Direction::UP => {
                        direction = Direction::LEFT;
                    },
                    Direction::DOWN => {
                        direction = Direction::RIGHT;
                    },
                }
                (x, y) = next_tile(lines, x, y, &direction)?;
            }
            '-' => {
                match direction {
                    Direction::UP | Direction::DOWN => {
                        // Walk left
                        let next_left = next_tile(lines, x, y, &Direction::LEFT);
                        if let Ok((xx, yy)) = next_left {
                            beam_walk(lines, xx, yy, Direction::LEFT);
                        }

                        // Walk right
                        direction = Direction::RIGHT;
                    }
                    _ => (),
                }
                (x, y) = next_tile(lines, x, y, &direction)?;
            },
            '|' => {

                match direction {
                    Direction::LEFT | Direction::RIGHT => {
                        // Walk up
                        let next_up = next_tile(lines, x, y, &Direction::UP);
                        if let Ok((xx, yy)) = next_up {
                            beam_walk(lines, xx, yy, Direction::UP);
                        }


                        // Walk down
                        direction = Direction::DOWN;
                    }
                    _ => (),
                }
                (x, y) = next_tile(lines, x, y, &direction)?;

            }

            _ => break,
        }

    }
    Ok((x, y))
}

fn part1() {

    let input = include_str!("input.txt");

    let mut lines: Vec<_> = input.split("\n")
        .map(|x| x.trim().chars()
            .map(|y| Tile::new(y)).collect::<Vec<_>>()
        )
        .collect();

    print_type_of(&lines);


    print_tiles(&lines);

    match beam_walk(&mut lines, 0, 0, Direction::RIGHT) {
        Ok(_) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }

    print_tiles(&lines);
    print_energized_tiles(&lines);
    
    let energized_tiles = {
        let mut count: u32 = 0;
        for line in &lines {
            for tile in line {
                if tile.energized{
                    count += 1;
                }
            }
        }
        count
    };

    println!("Energized tiles: {}", energized_tiles);

}

fn parse_input(filename: &str) -> Vec<Vec<Tile>>{
    let input = fs::read_to_string(filename).expect("Unable to read file");

    let mut lines: Vec<_> = input.split("\n")
        .map(|x| x.trim().chars()
            .map(|y| Tile::new(y)).collect::<Vec<_>>()
        )
        .collect();

    lines
}

fn part2() {
    let mut lines = parse_input("./src/input.txt");

    // print_type_of(&lines);
    // print_tiles(&lines);

    let mat_size = lines.len();

    let range_zero: Vec<usize> = vec![0; mat_size];
    let range_max: Vec<usize> = vec![mat_size-1; mat_size];
    let range_inc: Vec<usize> = (0..mat_size).collect();

    let start_vecors = [
        (
            Direction::LEFT, // direction
            range_zero.iter().zip(range_inc.iter()) // start coordinates
        ),
        (
            Direction::RIGHT, // direction
            range_max.iter().zip(range_inc.iter()) // start coordinates
        ),
        (
            Direction::UP, // direction
            range_inc.iter().zip(range_max.iter()) // start coordinates
        ),
        (
            Direction::DOWN, // direction
            range_inc.iter().zip(range_zero.iter()) // start coordinates
        ),
    ];

    let mut hightest_energized = 0;
    for (direction, start_coords) in start_vecors{
        for (x, y) in start_coords{

            let mut local_lines = parse_input("./src/input.txt");

            match beam_walk(&mut local_lines, *x, *y, direction) {
                Ok(_) => (), // println!("Success"),
                Err(e) => () //println!("Error: {}", e),
            }
        
            // Count energized tiles
            let energized_tiles = {
                let mut count: u32 = 0;
                for line in &local_lines {
                    for tile in line {
                        if tile.energized{
                            count += 1;
                        }
                    }
                }
                count
            };
            hightest_energized = cmp::max(hightest_energized, energized_tiles);

            //print_tiles(&local_lines);
            //print_energized_tiles(&local_lines);
            // println!("{:?}, {}, {}: {}", direction, *x, *y, energized_tiles);
        }
    }

    println!("Max energized tiles: {}", hightest_energized);

}

fn main() {

    part2();

}
