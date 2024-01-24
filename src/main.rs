use std::str::FromStr;

#[derive(PartialEq)]
#[derive(Debug)]
struct Isbn {
    raw: String,
    digits: Vec<u8>,
}

#[derive(Debug)]
#[derive(PartialEq)]
enum InvalidIsbn {
    TooLong,
    TooShort,
    FailedChecksum,
    InvalidCharacter(usize, char),
}

impl FromStr for Isbn {
    type Err = InvalidIsbn;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // his version
        let mut digits = Vec::with_capacity(15);
        for (i, c) in s.char_indices() {
            match c {
                '-' => continue,
                '0'..='9' => digits.push(c.to_digit(10).unwrap() as u8),
                _ => { return Err(InvalidIsbn::InvalidCharacter(i, c)); },
            }
        }

        let n_digits = digits.len();
        if n_digits > 13 {
            return Err(InvalidIsbn::TooLong);
        } else if n_digits < 13 {
            return Err(InvalidIsbn::TooShort);
        }

        if digits[12] != calculate_check_digit(&digits) {
            return Err(InvalidIsbn::FailedChecksum);
        }

        Ok(Isbn { raw: s.to_string(), digits: digits })

        // my version
        // let trimmed_s = s.trim_matches('-');
        // let mut digits = "".to_string();
        // for character in trimmed_s.chars() {
        //     if character.is_numeric() {
        //         // string version
        //         digits = digits + &character.to_string();
        //
        //     }
        // }
        //
        // // I thought the instructions said to drop the last zero if it ends in zero, but it looked like he was assuming we'd always drop the last digit.
        // // looks like the final digit of the ISBN is the check digit.
        // let (isbn_string, checksum_string) = digits.split_at(digits.len() - 1);
        // let checksum = checksum_string.parse::<u8>().unwrap();
        // let mut isbn_digits: Vec<u8> = vec![];
        // for char in isbn_string.chars() {
        //     isbn_digits.push(char.to_string().parse::<u8>().unwrap())
        // };
        //
        // if calculate_check_digit(&isbn_digits) != checksum {
        //     FailedChecksum
        // } else {
        //     Ok(Self { raw: s.to_string(), digits: isbn_digits })
        // }

    }
}

impl std::fmt::Display for Isbn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}

// https://en.wikipedia.org/wiki/International_Standard_Book_Number#ISBN-13_check_digit_calculation
fn calculate_check_digit(digits: &[u8]) -> u8 {
    // my version. I used a simpler array and modulo arithmetic to select the weight.
    let weights = [1, 3];
    let mut weights_sum = 0;
    for index in 0..digits.len() {
        weights_sum += digits[index] * weights[index % 2]
    }
    let check = 10 - (weights_sum % 10);
    if check == 10 {
        0
    } else {
        check
    }

    // This is the official version. The iterator and zipping  makes sense, but my version is slightly more compact
    // const WEIGHTS: [u8; 12] = [1, 3, 1, 3, 1, 3, 1, 3, 1, 3, 1, 3];
    //
    // let weight_applied: u32 = digits
    //     .iter()
    //     .zip(WEIGHTS.iter())
    //     .map(|(&x, &y)| x * y)
    //     .map(|subtotal| subtotal as u32)
    //     .sum();
    //
    // let check_digit = 10 - (weight_applied % 10);
    // match check_digit {
    //     10 => 0_u8,
    //     x => x as u8,
    // }
}

fn main() {
    let rust_in_action: Isbn = "978-3-16-148410-0".parse().unwrap();

    println!("Rust in Action's ISBN-13 ({}) is valid!", rust_in_action);
}

#[test]
fn can_correctly_calculate_check_digits() {
    let cases = [
        ([9_u8, 7, 8, 1, 8, 6, 1, 9, 7, 8, 7, 6], 9_u8),
        ([9_u8, 7, 8, 3, 1, 6, 1, 4, 8, 4, 1, 0], 0_u8),
    ];

    for (case, check) in cases.iter() {
        let actual = calculate_check_digit(case);
        println!("{:?} -> {}?  {}", &case, check, actual);
        assert_eq!(calculate_check_digit(case), *check)
    }
}

#[test]
fn rust_in_action() {
    let _: Isbn = "978-3-16-148410-0".parse().unwrap();
}

#[test]
fn bad_isbn() {
    let check: Result<Isbn, InvalidIsbn> = Isbn::from_str("78-3-16-148410-0");
    assert_eq!(check, Err(InvalidIsbn::TooShort));
}
