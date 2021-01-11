#[aoc(day3, part1)]
fn day3_1(input: &str) -> u32 {
    day3_main(input, 3, 1)
}

#[aoc(day3, part2)]
fn day3_2(input: &str) -> u32 {
    day3_main(input, 1, 1)
        * day3_main(input, 3, 1)
        * day3_main(input, 5, 1)
        * day3_main(input, 7, 1)
        * day3_main(input, 1, 2)
}

fn day3_main(input: &str, mvt_horizontal: u32, mvt_vertical: u32) -> u32 {
    let mut rows = input.lines();

    let nb_columns = 30;

    let mut current_index = 0;
    let mut res = 0;

    //let mut debug_i = 1;

    while let Some(row) = rows.next() {
        match row.chars().nth(current_index) {
            Some(char) => {
                if char == '#' {
                    res += 1;
                }
                //println!("Row = {}, Char = {}, Index = {}, Nb Trees = {}",debug_i, char, current_index, res);
                //debug_i +=1;
            }
            None => break,
        }

        current_index += mvt_horizontal as usize;
        if current_index > nb_columns {
            current_index -= nb_columns + 1;
        }

        for _i in 1..mvt_vertical {
            rows.next();
        }
    }

    return res;
}
