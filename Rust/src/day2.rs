#[aoc(day2, part1)]
fn day2_1(input: &str) -> u16 {
    let lines = input.lines();

    let mut res = 0;
    for line in lines {
        let mut split_by_spaces = line.split_whitespace();
        let mut min_max_split = split_by_spaces.next().unwrap().split('-');

        let min = min_max_split.next().unwrap().parse::<u32>().unwrap();
        let max = min_max_split.next().unwrap().parse::<u32>().unwrap();

        let char_for_test = split_by_spaces.next().unwrap().chars().nth(0).unwrap();

        let nb_char = split_by_spaces
            .next()
            .unwrap()
            .matches(char_for_test)
            .count() as u32;

        if min <= nb_char && nb_char <= max {
            res += 1;
        }
    }

    res
}

#[aoc(day2, part2)]
fn day2_2(input: &str) -> u16 {
    let lines = input.lines();

    let mut res = 0;
    for line in lines {
        let mut split_by_spaces = line.split_whitespace();
        let mut min_max_split = split_by_spaces.next().unwrap().split('-');

        let first_space = min_max_split.next().unwrap().parse::<u32>().unwrap();
        let second_space = min_max_split.next().unwrap().parse::<u32>().unwrap();

        let char_for_test = split_by_spaces.next().unwrap().chars().nth(0).unwrap();

        let password = split_by_spaces.next().unwrap();

        if second_space <= (password.len() as u32) {
            let mut nb_char_found =
                if password.chars().nth((first_space - 1) as usize).unwrap() == char_for_test {
                    1
                } else {
                    0
                };

            nb_char_found +=
                if password.chars().nth((second_space - 1) as usize).unwrap() == char_for_test {
                    1
                } else {
                    0
                };

            if nb_char_found == 1 {
                res += 1
            };
        }
    }

    res
}
