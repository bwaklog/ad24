use std::{char, env};

use ad24::Input;

#[derive(Debug, Clone)]
enum Move {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
struct Guard {
    i: i32,
    j: i32,
    direction: Move,
}

fn main() {
    let input = Input::init(6, env::args().nth(1).unwrap()).unwrap();

    let (mut location, mut grid): (Guard, Vec<Vec<char>>) =
        generate_grid_and_guard(input.content.clone());

    let dimensions = (grid.len(), grid[0].len());

    loop {
        if exit_map_condition(&location, dimensions) {
            break;
        }
        update_direction(&mut location, &mut grid);
    }

    let unique_locations: Vec<char> = grid
        .clone()
        .into_iter()
        .flatten()
        .filter(|c| ['X'].contains(c))
        .collect();

    (location, grid) = generate_grid_and_guard(input.content);

    let mut global_col_locations: Vec<(i32, i32)> = Vec::new();

    'mainloop: loop {
        if exit_map_condition(&location, dimensions) {
            break 'mainloop;
        }

        let mut location_clone = location.clone();
        let mut grid_clone = grid.clone();


        let (success, i, j) = try_create_obstacle_ahead(&location_clone, &mut grid_clone);
        if success && !global_col_locations.contains(&(i, j)) {
            eprintln!("Trying to insert O @ ({i}, {j})");

            let mut collisions: Vec<(usize, usize)> = Vec::new();

            'discover: loop {
                if exit_map_condition(&location_clone, dimensions) {
                    break 'discover;
                }

                if !update_direction_acyclic(
                    &mut location_clone,
                    &mut grid_clone,
                    &mut collisions,
                ) {
                    global_col_locations.push((i, j));
                    eprintln!("Discovered loop O @ ({i}, {j})");
                    break 'discover;
                }
            }
        }

        update_direction(&mut location, &mut grid);
    }

    println!(
        "{{\"day\": 6, \"distinct positions\": {}, \"fake_obstacles\": {} }}",
        unique_locations.len(),
        global_col_locations.len()
    );
}

fn generate_grid_and_guard(input: String) -> (Guard, Vec<Vec<char>>) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    // let mut collision_map: HashMap<(usize, usize), i32> = HashMap::new();

    let mut location: Guard = Guard {
        i: 0,
        j: 0,
        direction: Move::Up,
    };

    let rows: Vec<&str> = input.trim().lines().collect();

    for (i, _) in rows.iter().enumerate() {
        let mut temp: Vec<char> = Vec::new();
        for j in 0..rows[i].len() {
            let cell = rows[i].chars().nth(j).unwrap();
            if cell == '^' {
                location.i = i as i32;
                location.j = j as i32;
                temp.push('X');
            } else {
                temp.push(cell);
            }
        }
        grid.push(temp);
    }

    (location, grid)
}

fn move_location(location: &Guard) -> (i32, i32) {
    match location.direction {
        Move::Up => (location.i - 1, location.j),
        Move::Right => (location.i, location.j + 1),
        Move::Down => (location.i + 1, location.j),
        Move::Left => (location.i, location.j - 1),
    }
}

fn update_direction_acyclic(
    location: &mut Guard,
    grid: &mut [Vec<char>],
    // collision_map: &mut HashMap<(usize, usize), i32>,
    collisions: &mut Vec<(usize, usize)>,
) -> bool {
    let (i, j) = move_location(location);
    if grid[i as usize][j as usize] == '#' || grid[i as usize][j as usize] == '0' {

        if collisions.contains(&(i as usize, j as usize)) {
            return false;
        } else {
            collisions.push((i as usize, j as usize));
        }

        match location.direction {
            Move::Up => location.direction = Move::Right,
            Move::Right => location.direction = Move::Down,
            Move::Down => location.direction = Move::Left,
            Move::Left => location.direction = Move::Up,
        }
        return true;
    }

    location.i = i;
    location.j = j;
    true
}

fn try_create_obstacle_ahead(location: &Guard, grid: &mut [Vec<char>]) -> (bool, i32, i32) {
    let (i, j) = move_location(location);

    if grid[i as usize][j as usize] == '#' {
        return (false, i, j);
    }

    grid[i as usize][j as usize] = '0';
    (true, i, j)
}

fn update_direction(location: &mut Guard, grid: &mut [Vec<char>]) {
    let (i, j) = move_location(location);

    if grid[i as usize][j as usize] == '#' || grid[i as usize][j as usize] == '0' {
        grid[location.i as usize][location.j as usize] = 'X';
        match location.direction {
            Move::Up => location.direction = Move::Right,
            Move::Right => location.direction = Move::Down,
            Move::Down => location.direction = Move::Left,
            Move::Left => location.direction = Move::Up,
        }
        return;
    }

    grid[i as usize][j as usize] = 'X';
    location.i = i;
    location.j = j;
}

fn exit_map_condition(location: &Guard, dimensions: (usize, usize)) -> bool {
    match location.direction {
        Move::Up => location.i == 0,
        Move::Right => location.j as usize == dimensions.1 - 1,
        Move::Down => location.i as usize == dimensions.0 - 1,
        Move::Left => location.j == 0,
    }
}
