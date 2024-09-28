#[derive(Debug)]
pub struct User {
    pub username: String,
    pub email: String,
    pub active: bool,
    sign_count: u64,
}

impl User {
    pub fn new(username: &str, email: &str) -> User {
        User {
            username: String::from(username),
            email: String::from(email),
            active: true,
            sign_count: 0,
        }
    }
    pub fn sign_count(&self) -> u64 { // getter method
        self.sign_count
    }
}

pub struct Point(pub i32, pub i32, pub i32);
impl Point {
    pub fn norm(&self) -> f64 {
        let sum_of_squares = (self.0.pow(2) + self.1.pow(2) + self.2.pow(2)) as f64;
        sum_of_squares.sqrt()
    }
}

// unit like structs
pub struct UnitLike;

// struct LifeUser { //--- > this needs lifetimes
//     pub username: &str,
// }
