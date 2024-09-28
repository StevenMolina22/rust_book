pub fn _main_use() {
    println!("--- WELCOME TO THE PATTERN MATCH CRATE");
    let maybe_value = Some(5);

    // normal pattern matching
    //match maybe_value {
    //    Some(i) => println!("Num is: {}", i),
    //    None => (),
    //}

    // if let pattern matching
    if let Some(n) = maybe_value {
        println!("Pattern mached: {n}");
    }
}

pub fn _ref_use() {
    let maybe_value = Some(Box::new(5));
    println!("Ref pattern matching: ");

    match maybe_value {
        // matches agains the same item but not takes ownership
        Some(ref i) => println!("Num is: {i}"),
        None => println!("Nothing :("),
    }

    println!("Item: {}", maybe_value.expect("Item"));
}