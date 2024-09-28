pub fn _main() {
    // let s = id_str("Hello from lifetimes", "Second lifetime str");
    // {
    //     s = _id_str("Welcome to lifetimes, from inner scope");
    //     println!("{s:?}");
    // }
    let s1 = String::from("Hello from");
    let s2 = String::from("Lifetimes");
    let s;
    {
        // let s2 = String::from("Lifetimes");
        s = life_str(s1.as_str(), s2.as_str());
    }
    println!("{s:?}");
}

fn _id_str(s1: &str) -> &str {
    s1
}

fn life_str<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // chooses smaller str
    if s1.len() < s2.len() {
        s1
    } else {
        s2
    }
}
