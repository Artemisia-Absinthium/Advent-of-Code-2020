#[aoc(day3, part1)]
fn day3_1(input: &str) -> u32 {
    let mut rows = input.lines();

    let mvt_horizontal = 3;
    let mvt_vertical = 1;

    let nb_columns = 30;

    let mut current_index = 0;
    let mut res = 0;

    let mut debug_i = 1;

    while let Some(row) = rows.next() {
        match row.chars().nth(current_index) {
            Some(char) => {
                if char == '#' {
                    res += 1;
                }
                println!("Row = {}, Char = {}, Index = {}, Nb Trees = {}",debug_i, char, current_index, res);
                debug_i +=1;
            }
            None => break,
        }

        current_index += mvt_horizontal;
        if current_index > nb_columns {
            current_index -= nb_columns - 1;
        }
    }

    return res;
}
