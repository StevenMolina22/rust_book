pub fn print_mut(vec: &mut Vec<i32>) {
    if vec[0] > 100 {
        vec[0] = 100;
    }
    for i in vec {
        println!("{}", i)
    }
}

pub fn print_notmut(vec: &Vec<i32>) {
    for i in vec {
        println!("{}", i)
    }
}