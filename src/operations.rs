pub fn increment(counter: i32) -> i32 {
    counter = counter + 1;
    return counter;
}

pub fn decrement(counter: i32) -> i32 {
    counter = counter - 1;
    return counter;
}

pub fn ten_increment(counter: i32) -> i32 {
    for _i in 0..10 {
        increment(counter);
    }
}

pub fn ten_decrement(counter: i32) -> i32 {
    for _i in 0..10 {
        decrement(counter);
    }
}
