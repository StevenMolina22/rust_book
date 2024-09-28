pub fn _main() {
    r_for(0, 3, |x| println!("{x}"));

    println!("Result of r sum: {}", sum(&[1, 2, 3, 4, 5]));
    assert_eq!(is_palindrome("hello"), false);
    assert_eq!(is_palindrome("lisil"), true);

    let mut s = String::from("hello");
    reverse_str(&mut s);
    println!("{s}");
}

fn r_for(i: i32, j: i32, f: impl Fn(i32)) {
    if i == j {
        return;
    }
    f(i);
    r_for(i + 1, j, f);
}

fn sum(v: &[i32]) -> i32 {
    match v {
        [] => 0,
        v => sum(&v[1..]) + v[0],
    }
}

fn is_palindrome(s: &str) -> bool {
    if s.len() <= 1 {
        return true;
    }
    s.chars().next() == s.chars().last() && is_palindrome(&s[1..s.len() - 1])
}

fn reverse_str(s: &mut String) {
    if s.len() <= 1 {
        return;
    }

    let first = s.remove(0);
    let last = s.pop().unwrap();

    s.insert(0, last);
    s.push(first);
    reverse_str(&mut s[1..s.len()-1].to_string());
}

fn permute(s: &str) {}
