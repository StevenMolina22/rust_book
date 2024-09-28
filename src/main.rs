use functional::enums::{get_salary, SportPlayer};

mod data_types;
mod literals;
mod printer;
mod structs;
mod users;
mod functional;
mod ownership;

fn main() {
    println!("--- WELCOME TO THE MAIN CRATE ---");
    // data_types::_tuple_structs();
    // data_types::_mut_lifetime();
    // pattern_match::_main_use();
    // pattern_match::_ref_use();
    // ownership::rc_test::_use();
    // functional::closures::_use();
    // literals::_use();
    functional::expressions::_main();
    ownership::lifetimes::_main();
    functional::recursion::_main();

    println!("Salary: {}", get_salary(SportPlayer::Basketball));
}
