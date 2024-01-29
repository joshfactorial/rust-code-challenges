use chrono::NaiveDate;

/// Parses a string that represents a date. When a date
/// is unable to be determined, return `None`. 
fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
    // official solution
    let text = text.trim();
    if !text.bytes().any(|x| x.is_ascii_digit()) {
        return None;
    }

    // my solution was somewhat hamfisted
    // I tacked a - at the end to get my loop to execute a final time
    let text_to_parse = text.to_string() + "-";

    // This will store the text format for NaiveDate parsing
    let mut text_fmt = "".to_string();
    // These are the potential delimiters I expect. Space is not valid, so if I see it it's a hard kill
    let recognized_delimiters = ['/', '.', '-', ' '];
    // To store the particular delimiter of this input.
    let mut current_delimiter: Option<char> = None;

    // We know we'll have three pieces, so each "position" will be on component (year, month or day)
    let mut symbol_position = 0;

    // to store the current symbol
    let mut temp_symbol = String::with_capacity(9);
    // This will help us decide if a 2-digit number is a month or a day
    let mut found_month = false;

    // This will give us a vector of parts for us to infer the format.
    for symbol in text_to_parse.chars() {
        if recognized_delimiters.contains(&symbol) {
            // We reached a delimiter, we'll add this block to our date vector, reset temp_symbol and confirm that
            // the input isn't screwy.

            // First store the delimiter. Skip this step if already done
            if symbol == ' ' {
                // spaces aren't a valid delimiter for this exercise
                return None;
            } else {
                // First delimiter encountered.
                current_delimiter = Some(symbol);
            }

            // This checks the length and position. Length 4 is always Year, length 3 is always month, but length
            // 2 could be the day or the month, so we check where it is and if we have already found the month.
            match temp_symbol.len() {
                4 => text_fmt += &format!("%Y{}", current_delimiter.unwrap().to_string()),
                3 => { found_month = true; text_fmt += &format!("%b{}", current_delimiter.unwrap().to_string()) },
                2 => {
                    match symbol_position {
                        // First in the sequence, then it's the day
                        0 => text_fmt += &format!("%d{}", current_delimiter.unwrap().to_string()),
                        1 => {
                            // Second in the sequence, month if we haven't found month yet, otherwise day
                            if found_month == true {
                                text_fmt += &format!("%d{}", current_delimiter.unwrap().to_string())
                            } else {
                                text_fmt += &format!("%m{}", current_delimiter.unwrap().to_string())
                            }
                        },
                        // If it's at the end, it's always the day
                        2 => text_fmt += &format!("%d{}", current_delimiter.unwrap().to_string()),
                        // Any more than 3 symbols makes this an invalid date
                        _ => return None,
                    }
                },
                // If our symbol is 1-character or >4 characters long, it's invalid
                _ => return None,
            }

            // Reset temp symbol and increment our position
            temp_symbol = String::with_capacity(9);
            symbol_position += 1;
        } else {
            // Build the symbol until you see a delimiter
            temp_symbol.push(symbol)
        }
    }

    // We end up with an extra delimiter, which we strip off here
    text_fmt = text_fmt.strip_suffix(current_delimiter.unwrap()).unwrap().to_string();

    Some(NaiveDate::parse_from_str(text, &text_fmt).unwrap())
}

fn main() {
    let dates = [
        "2010-12-11",
        "1999/Mar/02",
        "01.Mar.2021",
        "Mar.05.2021",
        "not a date",
    ];

    for d in dates.iter() {
        println!("{} -> {:?}", d, flexible_date_parse(d));
    }

}

#[test]
fn ymd_hyphen() {
    assert_eq!(flexible_date_parse("2010-12-11"), Some(NaiveDate::from_ymd(2010, 12, 11)))
}

#[test]
fn ymd_slash() {
    assert_eq!(flexible_date_parse("1999/Mar/02"), Some(NaiveDate::from_ymd(1999, 3, 2)))
}

#[test]
fn dmy_dot() {
    assert_eq!(flexible_date_parse("01.Mar.2021"), Some(NaiveDate::from_ymd(2021, 3, 1)))
}

#[test]
fn mdy_dot() {
    assert_eq!(flexible_date_parse("Apr.05.2021"), Some(NaiveDate::from_ymd(2021, 4, 5)))
}

#[test]
fn invalid() {
    assert_eq!(flexible_date_parse("not a date"), None)
}


