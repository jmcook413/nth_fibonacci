use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let fibonacci_index: u128 = args[1].trim().parse()
        .expect("Please enter a positive integer greater than 0!");

    find_the_number(fibonacci_index);
}

fn find_the_number(index: u128) {
    let mut f_num: u128 = 0;
    let mut f_in: u128 = 1;
    let mut f_temp: u128;

    for _i in 1..index {
        f_temp = f_num + f_in;

        f_num = f_in;
        f_in = f_temp;
    }

    println!("The fibonacci number with an index of {} is {}.", index, f_num);
}
