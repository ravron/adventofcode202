use regex::Regex;

pub fn day4() {
    let input = include_str!("../inputs/day4.txt");

    let mut valid_passports_1 = 0;

    for record in input.split("\n\n") {
        if has_required_fields(record) {
            valid_passports_1 += 1;
        }
    }

    println!("part 1: {} valid \"passports\"", valid_passports_1);

    let mut valid_passports_2 = 0;

    'outer: for record in input.split("\n\n") {
        if !has_required_fields(record) { continue; }

        for field in record.split_whitespace() {
            let (field_name, field_value) = {
                let sp: Vec<&str> = field.split(':').collect();
                (sp[0], sp[1])
            };

            if !validate_field(field_name, field_value) {
                println!("{}:{} is invalid", field_name, field_value);
                continue 'outer;
            }
        }

        valid_passports_2 += 1;
    }

    println!("part 2: {} valid \"passports\"", valid_passports_2);
}

fn has_required_fields(record: &str) -> bool {
    let required_fields = vec![
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
        // "cid",
    ];
    lazy_static! {
        static ref RE_1: regex::Regex = Regex::new(r"([a-z]+):").unwrap();
    }

    let fields: Vec<&str> = RE_1.captures_iter(record)
        .map(|cap| cap.get(1).unwrap().as_str())
        .collect();
    required_fields.iter().all(|rf| fields.contains(rf))
}

fn validate_field(field_name: &str, field_value: &str) -> bool {
    lazy_static!{
        static ref HCL_RE: regex::Regex = Regex::new(r"#[a-z0-9]{6}").unwrap();
    }
    let valid_ecl = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    match field_name {
        "byr" => field_value.len() == 4 &&
            (1920..=2020).contains(&field_value.parse::<i32>().unwrap_or_default()),
        "iyr" => field_value.len() == 4 &&
            (2010..=2020).contains(&field_value.parse::<i32>().unwrap_or_default()),
        "eyr" => field_value.len() == 4 &&
            (2020..=2030).contains(&field_value.parse::<i32>().unwrap_or_default()),
        "hgt" => if let Some(scalar) = field_value.strip_suffix("cm") {
                (150..=193).contains(&scalar.parse::<i32>().unwrap_or_default())
            } else if let Some(scalar) = field_value.strip_suffix("in") {
                (59..=76).contains(&scalar.parse::<i32>().unwrap_or_default())
            } else {
                false
            },
        "hcl" => HCL_RE.is_match(field_value),
        "ecl" => valid_ecl.contains(&field_value),
        "pid" => field_value.len() == 9 && field_value.parse::<i32>().is_ok(),
        "cid" => true,

        _ => false,
    }
}