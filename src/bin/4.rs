use ad24::Input;

type MainResult<T> = Result<T, Box<dyn std::error::Error>>;

const XMAS_LENI: usize = "xmas".len() - 1;
const XMAS_STR: &str = "XMAS";

fn main() -> MainResult<()> {
    let input = Input::init(4, std::env::args().nth(1).unwrap()).unwrap();

    let mut map: Vec<Vec<char>> = Vec::with_capacity(1000);

    input.content.trim().lines().for_each(|line| {
        let mut temp = Vec::new();
        for char in line.chars() {
            temp.push(char);
        }
        map.push(temp);
    });

    let mut key_pos: Vec<(usize, usize)> = Vec::new();

    for (i, _) in map.iter().enumerate() {
        for j in 0..map[i].len() {
            if map[i][j] == 'X' {
                // total += check_surroundings(&map, i, j);
                key_pos.push((i, j));
            }
        }
    }

    let total = check_surroundings(&map, key_pos);

    let x_mas_total = check_x_mas(&map);

    println!("{{\"part 1\": {}, \"part 2\": {}}}", total, x_mas_total);

    Ok(())
}

fn check_surroundings(map: &[Vec<char>], key_pos: Vec<(usize, usize)>) -> u32 {
    let mut total = 0;

    for pos in key_pos {
        total += check_horizontal(map, pos.0, pos.1)
            + check_vertical(map, pos.0, pos.1)
            + check_diagonal(map, pos.0, pos.1);
    }

    total
}

fn check_horizontal(map: &[Vec<char>], i: usize, j: usize) -> u32 {
    let mut total = 0;

    if j + XMAS_LENI < map[i].len()
        && format!(
            "{}{}{}{}",
            map[i][j],
            map[i][j + 1],
            map[i][j + 2],
            map[i][j + 3]
        ) == XMAS_STR
    {
        total += 1;
    }

    if j >= XMAS_LENI
        && format!(
            "{}{}{}{}",
            map[i][j],
            map[i][j - 1],
            map[i][j - 2],
            map[i][j - 3]
        ) == XMAS_STR
    {
        total += 1;
    }

    total
}

fn check_vertical(map: &[Vec<char>], i: usize, j: usize) -> u32 {
    let mut total = 0;

    if i + XMAS_LENI < map.len()
        && format!(
            "{}{}{}{}",
            map[i][j],
            map[i + 1][j],
            map[i + 2][j],
            map[i + 3][j]
        ) == XMAS_STR
    {
        total += 1;
    }

    if i >= XMAS_LENI
        && format!(
            "{}{}{}{}",
            map[i][j],
            map[i - 1][j],
            map[i - 2][j],
            map[i - 3][j]
        ) == XMAS_STR
    {
        total += 1;
    }

    total
}

fn check_diagonal(map: &[Vec<char>], i: usize, j: usize) -> u32 {
    let mut total = 0;

    if i + XMAS_LENI < map.len()
        && j + XMAS_LENI < map[i].len()
        && format!(
            "{}{}{}{}",
            map[i][j],
            map[i + 1][j + 1],
            map[i + 2][j + 2],
            map[i + 3][j + 3]
        ) == XMAS_STR
    {
        total += 1;
    }

    if i + XMAS_LENI < map.len()
        && j >= XMAS_LENI
        && format!(
            "{}{}{}{}",
            map[i][j],
            map[i + 1][j - 1],
            map[i + 2][j - 2],
            map[i + 3][j - 3]
        ) == XMAS_STR
    {
        total += 1;
    }

    if i >= XMAS_LENI
        && j + XMAS_LENI < map[i].len()
        && format!(
            "{}{}{}{}",
            map[i][j],
            map[i - 1][j + 1],
            map[i - 2][j + 2],
            map[i - 3][j + 3]
        ) == XMAS_STR
    {
        total += 1;
    }

    if i >= XMAS_LENI
        && j >= XMAS_LENI
        && format!(
            "{}{}{}{}",
            map[i][j],
            map[i - 1][j - 1],
            map[i - 2][j - 2],
            map[i - 3][j - 3]
        ) == XMAS_STR
    {
        total += 1;
    }

    total
}

// These elves are amazing
fn check_x_mas(map: &[Vec<char>]) -> u32 {
    let mut total = 0;
    for i in 1..map.len() - 1 {
        for j in 1..map[i].len() - 1 {
            if map[i][j] == 'A' && check_possible_diagonal(map, i, j) {
                total += 1;
            }
        }
    }

    total
}

fn check_possible_diagonal(map: &[Vec<char>], i: usize, j: usize) -> bool {
    let forward = "MAS";
    let reverse = "SAM";

    let d1 = format!("{}{}{}", map[i - 1][j - 1], map[i][j], map[i + 1][j + 1]);
    let d2 = format!("{}{}{}", map[i + 1][j - 1], map[i][j], map[i - 1][j + 1]);

    (d1 == forward || d1 == reverse) && (d2 == forward || d2 == reverse)
}
