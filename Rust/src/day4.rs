struct Passport {
    byr: Option<u32>,
    iyr: Option<u32>,
    eyr: Option<u32>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<u32>
}

#[aoc_generator(day4)]
fn day4_parsing(input: &str) -> Vec<Passport> {
    let cleaned_input = input.replace("\n\n","-").replace("\n"," ");

    let mut passports = Vec::new();

    let mut first_split = cleaned_input.split("-");
    while let Some(passport) = first_split.next() {

        let mut byr = None::<u32>;
        let mut iyr = None::<u32>;
        let mut eyr = None::<u32>;
        let mut hgt = None::<String>;
        let mut hcl = None::<String>;
        let mut ecl = None::<String>;
        let mut pid = None::<String>;
        let mut cid = None::<u32>;

        let mut passport_values = passport.split_whitespace();
        while let Some(current_value) = passport_values.next() {
            
            let mut splited_value = current_value.split(":");
            match splited_value.next() {

                Some(value_header) => {
                    match splited_value.next() {

                        Some(value) => {
                            match value_header {
                                "byr" => {
                                    byr = Some(value.parse::<u32>().unwrap());
                                }
                                "iyr" => {
                                    iyr = Some(value.parse::<u32>().unwrap());
                                }
                                "eyr" => {
                                    eyr = Some(value.parse::<u32>().unwrap());
                                }
                                "hgt" => {
                                    hgt = Some(value.to_string());
                                }
                                "hcl" => {
                                    hcl = Some(value.to_string());
                                }
                                "ecl" => {
                                    ecl = Some(value.to_string());
                                }
                                "pid" => {
                                    pid = Some(value.to_string());
                                }
                                "cid" => {
                                    cid = Some(value.parse::<u32>().unwrap());
                                }
                                _ => break,
                            }
                        }
                        None => break,
                    }
                }
                None => break,
            }
        }

        passports.push(Passport{byr, iyr, eyr, hgt, hcl, ecl, pid, cid});
    }

    passports
}

#[aoc(day4, part1)]
fn day4_part1(input: &Vec<Passport>) -> u32 {
    let mut nb_valid_passport = 0;
    for passport in input {
        if passport.byr != None
        && passport.iyr != None
        && passport.eyr != None
        && passport.hgt != None
        && passport.hcl != None
        && passport.ecl != None
        && passport.pid != None{
            nb_valid_passport += 1;
        }
    }

    nb_valid_passport
}