pub fn negative_spell(n: i64) -> String {
    match n {
        0 => String::from("zero"),
        n if n > 0 => String::from("error: positive number"),
        _ => {
            let positive_n = n.abs() as u64;
            format!("minus {}", spell(positive_n))
        }
    }
}

pub fn spell(n: u64) -> String {
    if n == 0 {
        String::from("zero")
    } else {
        spelling(n).trim().to_string()
    }
}

fn spelling(n: u64) -> String {
    match n {
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        15 => String::from("fifteen"),
        14 | 16 | 17 | 18 | 19 => spelling(n % 10) + "teen",
        20 => String::from("twenty"),
        21..=29 => "twenty-".to_owned() + &spelling(n % 10),
        30 => String::from("thirty"),
        31..=39 => "thirty-".to_owned() + &spelling(n % 10),
        40 => String::from("forty"),
        41..=49 => "forty-".to_owned() + &spelling(n % 10),
        50 => String::from("fifty"),
        51..=59 => "fifty-".to_owned() + &spelling(n % 10),
        60 => String::from("sixty"),
        61..=69 => "sixty-".to_owned() + &spelling(n % 10),
        70 => String::from("seventy"),
        71..=79 => "seventy-".to_owned() + &spelling(n % 10),
        80 => String::from("eighty"),
        81..=89 => "eighty-".to_owned() + &spelling(n % 10),
        90 => String::from("ninety"),
        91..=99 => "ninety-".to_owned() + &spelling(n % 10),
        n if n >= 1_000_000 => spelling(n / 1_000_000) + " million " + &spelling(n % 1_000_000),
        n if n >= 1_000 => spelling(n / 1_000) + " thousand " + &spelling(n % 1_000),
        n if n >= 100 => spelling(n / 100) + " hundred " + &spelling(n % 100),
        _ => String::from(""),
    }
}

/* pub fn negative_spell(n: i64) -> String {
    match n {
        0 => String::from("zero"),
        n if n > 0 => String::from("error: positive number"),
        _ => format!("minus {}", spell(n.abs() as u64))
    }
}

fn spell(n: u64) -> String {
    if n == 0 {
        return String::from("zero");
    }
    let result = match n {
        1..=19 => single_word(n),
        20..=99 => two_digit(n),
        100..=999 => three_digit(n),
        1_000..=999_999 => {
            let thousands = spell(n / 1_000);
            let remainder = spell(n % 1_000);
            if remainder == "zero" {
                format!("{} thousand", thousands)
            } else {
                format!("{} thousand {}", thousands, remainder)
            }
        },
        _ => {
            let millions = spell(n / 1_000_000);
            let remainder = spell(n % 1_000_000);
            if remainder == "zero" {
                format!("{} million", millions)
            } else {
                format!("{} million {}", millions, remainder)
            }
        },
    };
    result.trim().to_string()
}

fn single_word(n: u64) -> String {
    match n {
        1 => "one", 2 => "two", 3 => "three", 4 => "four", 5 => "five",
        6 => "six", 7 => "seven", 8 => "eight", 9 => "nine", 10 => "ten",
        11 => "eleven", 12 => "twelve", 13 => "thirteen", 14 => "fourteen", 
        15 => "fifteen", 16 => "sixteen", 17 => "seventeen", 18 => "eighteen", 
        19 => "nineteen",
        _ => unreachable!(),
    }.to_string()
}

fn two_digit(n: u64) -> String {
    let tens = ["twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    let (ten, one) = ((n / 10) as usize - 2, n % 10);
    match one {
        0 => tens[ten].to_string(),
        _ => format!("{}-{}", tens[ten], single_word(one))
    }
}

fn three_digit(n: u64) -> String {
    let hundreds = n / 100;
    let remainder = n % 100;
    if remainder == 0 {
        format!("{} hundred", single_word(hundreds))
    } else {
        format!("{} hundred {}", single_word(hundreds), spell(remainder))
    }
} */