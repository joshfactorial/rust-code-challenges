use chrono::{Local, TimeZone, Date, Datelike};

struct ImportantEvent {
    what:String,
    when:Date<Local>
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    fn is_passed(&self) -> bool {
        self.when < Local::today()
        // let now_time = Local::now();
        // let current_date: Date<Local> = Local.ymd(now_time.year(), now_time.month(), now_time.day());
        // if (current_date.year() > self.when.year())
        //     | (current_date.month() > self.when.month())
        //     | (current_date.day() > self.when.day()) {
        //     true
        // } else {
        //     false
        // }
    }
}

fn main() {
    let missed_christmas = ImportantEvent {
        what: String::from("Christmas"),
        when: Local.ymd(2023, 12, 25),
    };
    
    if missed_christmas.is_passed() {
        println!("oh well, maybe next year");
    } else {
        println!("☃︎");
    }
}

#[test]
fn in_past() {
    use chrono::Duration;

    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        when: Local::today() - Duration::hours(25),
    };

    assert!(event.is_passed())
}

#[test]
fn in_future() {
    use chrono::Duration;
    
    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        when: Local::today() + Duration::hours(25),
    };

    assert!(!event.is_passed())
}

