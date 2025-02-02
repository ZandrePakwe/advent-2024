use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs,
};

use strum::{EnumIter, IntoEnumIterator};

fn read_input(filename: &str) -> Maze {
    let input =
        fs::read_to_string(format!("src/day18/{filename}.txt")).expect("day 18 input missing");

    let coordinates = input
        .lines()
        .map(|line| {
            let numbers = line
                .split(',')
                .map(|number| number.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            return Coordinate {
                x: numbers[0],
                y: numbers[1],
            };
        })
        .collect();

    return Maze::new(coordinates);
}

#[derive(EnumIter, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(PartialEq)]
enum TileType {
    Empty,
    Filled,
}

struct Tile {
    tile_type: TileType,
    distance: usize,
}

#[derive(PartialEq, PartialOrd, Eq)]
struct TileToCheck {
    coordinate: Coordinate,
    cost: usize,
}

impl Ord for TileToCheck {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

struct Maze {
    tiles: HashMap<Coordinate, Tile>,
    memory_tiles: Vec<Coordinate>,
    max_x: usize,
    max_y: usize,
    memory_dropped: usize,
}

impl Maze {
    fn new(memory_tiles: Vec<Coordinate>) -> Self {
        let max_x = memory_tiles.iter().map(|tile| tile.x).max().unwrap();
        let max_y = memory_tiles.iter().map(|tile| tile.y).max().unwrap();

        let mut tiles = HashMap::new();

        for x in 0..=max_x {
            for y in 0..=max_y {
                let coordinate = Coordinate { x, y };
                let tile = Tile {
                    distance: usize::MAX,
                    tile_type: TileType::Empty,
                };
                tiles.entry(coordinate).insert_entry(tile);
            }
        }

        return Maze {
            max_x,
            max_y,
            memory_tiles,
            tiles,
            memory_dropped: 0,
        };
    }

    fn reset(&mut self) {
        for tile in self.tiles.iter_mut() {
            tile.1.distance = usize::MAX;
        }
    }

    fn find_max_available_time(&mut self) -> Coordinate {
        let mut shortest_path = self.dijkstra();
        self.reset();

        while shortest_path < usize::MAX {
            self.drop_memory(self.memory_dropped + 1);
            shortest_path = self.dijkstra();
            self.reset();
        }

        let coordinate_that_blocks = self.memory_tiles[self.memory_dropped - 1];

        return coordinate_that_blocks;
    }

    fn drop_memory(&mut self, count: usize) {
        self.memory_dropped = count;

        let memory_to_drop = self.memory_tiles[0..count].to_vec();

        for item in memory_to_drop {
            let tile = self.tiles.get_mut(&item).unwrap();
            tile.tile_type = TileType::Filled;
        }
    }

    fn draw(&self) {
        let mut string = String::new();

        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                let coordinate = Coordinate { x, y };
                let tile = self.tiles.get(&coordinate).unwrap();

                string += match tile.tile_type {
                    TileType::Empty => ".",
                    TileType::Filled => "#",
                }
            }
            string += "\n";
        }

        println!("{}", string);
    }

    fn dijkstra(&mut self) -> usize {
        let end = Coordinate {
            x: self.max_x,
            y: self.max_y,
        };

        let start = self.tiles.get_mut(&Coordinate { x: 0, y: 0 }).unwrap();
        start.distance = 0;

        let mut priority_heap = BinaryHeap::new();

        priority_heap.push(Reverse(TileToCheck {
            coordinate: Coordinate { x: 0, y: 0 },
            cost: 0,
        }));

        while let Some(Reverse(tile_to_check)) = priority_heap.pop() {
            let new_cost = tile_to_check.cost + 1;
            let tiles_to_check =
                get_next_coordinates(&tile_to_check.coordinate, self.max_y, self.max_x);

            for tile in tiles_to_check {
                let next_tile = self.tiles.get_mut(&tile).unwrap();

                if next_tile.tile_type == TileType::Empty && next_tile.distance > new_cost {
                    next_tile.distance = new_cost;
                    priority_heap.push(Reverse(TileToCheck {
                        coordinate: tile,
                        cost: new_cost,
                    }));
                }
            }
        }

        return self.tiles.get(&end).unwrap().distance;
    }
}

fn get_next_coordinates(coordinate: &Coordinate, y_max: usize, x_max: usize) -> Vec<Coordinate> {
    let mut result = vec![];

    for direction in Direction::iter() {
        match direction {
            Direction::Up => {
                let x = coordinate.x;
                let y = coordinate.y.checked_sub(1);

                if let Some(y) = y {
                    result.push(Coordinate { x, y });
                }
            }
            Direction::Down => {
                let y = coordinate.y + 1;
                let x = coordinate.x;
                if y <= y_max {
                    result.push(Coordinate { x, y });
                }
            }
            Direction::Left => {
                let y = coordinate.y;
                let x = coordinate.x.checked_sub(1);

                if let Some(x) = x {
                    result.push(Coordinate { x, y });
                }
            }
            Direction::Right => {
                let y = coordinate.y;
                let x = coordinate.x + 1;
                if x <= x_max {
                    result.push(Coordinate { x, y });
                }
            }
        }
    }

    return result;
}

pub fn day_18_part_1() {
    let mut maze = read_input("input");

    maze.drop_memory(1024);

    let shortest_path = maze.dijkstra();

    maze.draw();

    println!("shortest path: {}", shortest_path)
}

pub fn day_18_part_2() {
    let mut maze = read_input("input");

    maze.drop_memory(1024);

    maze.dijkstra();

    let coordinate_that_blocks = maze.find_max_available_time();

    maze.draw();

    println!(
        "Coordinate that blocks: {},{}",
        coordinate_that_blocks.x, coordinate_that_blocks.y
    )
}

#[test]
fn example_input() {
    let mut maze = read_input("example");

    maze.drop_memory(12);

    let shortest_path = maze.dijkstra();

    assert_eq!(shortest_path, 22)
}

#[test]
fn example_input_part_2() {
    let mut maze = read_input("example");

    maze.drop_memory(12);

    let coordinate_that_blocks = maze.find_max_available_time();

    assert_eq!(coordinate_that_blocks, Coordinate { x: 6, y: 1 })
}
