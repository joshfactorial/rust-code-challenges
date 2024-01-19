use chrono::NaiveDate;

fn weeks_between(a: &str, b: &str) -> i32 {

    // mine
    let a_date = NaiveDate::parse_from_str(a, "%Y-%m-%d");
    let b_date = NaiveDate::parse_from_str(b, "%Y-%m-%d");

    let elapsed_days = b_date.unwrap() - a_date.unwrap();
    (elapsed_days.num_days() / 7) as i32

}

fn main() {
    let n_weeks = weeks_between("2010-01-21", "2010-10-21");

    println!("hello: {}", n_weeks);
}

#[test]
fn same_day() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-10");
    assert_eq!(n_weeks, 0);
}

#[test]
fn one_week() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-18");
    assert_eq!(n_weeks, 1);
}

#[test]
fn past() {
    let n_weeks = weeks_between("1010-10-18", "1010-10-10");
    assert_eq!(n_weeks, -1);
}

#[test]
fn way_past() {
    let n_weeks = weeks_between("2023-01-01", "2024-01-03");
    assert_eq!(n_weeks, 52)
}