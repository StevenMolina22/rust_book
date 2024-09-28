use crate::structs::Point;

pub fn _mut_lifetime() {
    // heap variable
    let mut vec = vec![1, 2, 3];

    // reference to variable
    let ref1 = &vec;
    println!("{:?}", ref1);

    // mutates variable
    vec.push(3);

    let ref2 = &vec;
    println!("{:?}", ref2);

    let elem_ref = &vec[0];
    println!("{}", elem_ref);
}

pub fn _tuple_structs() {
    let mut p1 = (3, 2, 1);
    p1.0 += 1;

    let p = Point(3, 2, 1);

    println!("{}", p.norm());
}
