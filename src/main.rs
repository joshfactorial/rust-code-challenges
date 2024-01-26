
mod run_length_encoding {
    use std::str::FromStr;
    use std::iter;

    pub fn encode(text: &str) -> String {
        // My version.
        // I felt like I had to convert types back and forth a lot in this. It's probably not the most elegant solution
        // but it did pass the tests.
        // let mut current_count = 0;
        // let mut return_string = "".to_string();
        // // x is just a placeholder, this will get overwritten
        // let mut previous_char = 'x';
        // let mut first_time = true;
        // for letter in text.chars() {
        //     if first_time {
        //         previous_char = letter.clone();
        //         current_count += 1;
        //         first_time = false;
        //     } else if (letter == previous_char) & (current_count < 9){
        //         current_count += 1;
        //     } else {
        //         return_string += &format!("{}{}", current_count, previous_char.to_string());
        //         current_count = 1;
        //         previous_char = letter.clone()
        //     }
        // }
        // return_string += &format!("{}{}", current_count, previous_char.to_string());
        // return_string.to_string()

        // official solution
        // Essentially the same logic, but using that push_str method to make things easier than what I did
        let mut count = 0;
        let mut encoded = String::new();
        let mut prev: Option<char> = None;
        let mut chars = text.chars();

        while let Some(c) = chars.next() {
            if prev.is_none() {
                prev = Some(c);
            }

            let prev_ = prev.unwrap();
            if prev_ != c || (count == 9) {
                encoded.push_str(&format!("{}{}", count, prev_));
                count = 0;
            }
            count += 1;
            prev = Some(c);
        }

        if prev.is_some() {
            encoded.push_str(&format!("{}{}", count, prev.unwrap()));
        }
        encoded

    }
    
    pub fn decode(text: &str) -> String {
        // My version.
        // I felt like I had to convert types back and forth a lot in this. It's probably not the most elegant solution
        // but it did pass the tests.
        //original line
        // let mut return_string = "".to_string();
        // their line
        let mut return_string = String::with_capacity(text.len() * 2);

        let mut count = 0;

        // I used a for loop because I couldn't think how to make a while loop. But basically the difference ends up
        // being that I had to do a bunch of extra bullshit.
        // for letter in text.chars() {
        //     if letter.is_numeric() {
        //         count = usize::from_str(&*letter.to_string()).unwrap();
        //     } else {
        //         let x: String = iter::repeat(letter.to_string()).take(count).collect();
        //         return_string += &x;
        //     }
        // }

        // they used a Chars assignment to enable a while loop, whereas i did a for loop
        let mut chars = text.chars();

        while let (Some(n_raw), Some(c)) = (chars.next(), chars.next()) {
            let n = n_raw.to_digit(10).unwrap();
            for _ in 0..n {
                return_string.push(c);
            }
        }

        return_string.to_string()
    }
}

fn main() {
    use run_length_encoding::*;
    println!("{}", encode("Fuck you"));
    let my_text = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAbsolute baloons.";
    let test = encode(my_text);
    println!("{}", test);
    println!("{}", decode(&test));
    assert_eq!(my_text, decode(&test));
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    println!("{}", encode(input));
    assert_eq!(decode(&encode(input)), input);
}

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}
