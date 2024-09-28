pub fn print_mut(vec: &mut Vec<i32>) {
    println!("--- WELCOME TO THE PRINTER CRATE ---");
    if vec[0] > 100 {
        vec[0] = 100;
    }
    for i in vec {
        println!("{}", i)
    }
}

pub fn print_notmut(vec: &Vec<i32>) {
    println!("--- WELCOME TO THE PRINTER CRATE ---");
    for i in vec {
        println!("{}", i)
    }
}