fn sort_usernames<T: AsRef<str>>(usernames: &mut Vec<T>) {
    // My version
    usernames.sort_by(|x,y| x.as_ref().to_lowercase().partial_cmp(&y.as_ref().to_lowercase()).unwrap())

    // official version
    // usernames
    //     .sort_by_cached_key(|x: &T| x.as_ref().to_lowercase());
}

fn main() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];

    println!("unsorted: {:?}", &users);
    sort_usernames(&mut users);
    println!("sorted:   {:?}", &users);
}

#[test]
fn five_users() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
    let sorted = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
    sort_usernames(&mut users);

    assert_eq!(users, sorted);
}
