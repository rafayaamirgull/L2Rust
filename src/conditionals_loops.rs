fn check_is_male(is_male: bool) -> bool {
    if is_male {
        return true;
    } else {
        return false;
    }
}

fn conditionals() {
    let mut name: &str = "Rafay";
    let mut is_male: bool = true;
    if check_is_male(is_male) {
        println!("{} is masculine", name);
    } else {
        println!("{} is feminine", name);
    }

    name = "random_girl";
    is_male = false;
    if check_is_male(is_male) {
        println!("{} is masculine", name);
    } else {
        println!("{} is feminine", name);
    }
}

fn count_till_n_for(num: i32) {
    println!("\n\n===   Counting with FOR LOOP    ===");
    for i in 1..=num {
        println!("counter at: {}", i);
    }
}

fn count_till_n_while(num: i32) {
    let mut counter: i32 = 1;
    println!("\n\n===   Counting with WHILE LOOP    ===");
    while counter <= num {
        println!("counter at: {}", counter);
        counter += 1;
    }
}

fn iterate_over_str_meth1(my_str: &str) {
    for chara in my_str.chars() {
        println!("{} ", chara);
    }
}

fn iterate_over_str_meth2(my_str: &str) {
    for (_i, chara) in my_str.chars().enumerate() {
        println!("{} is at index {}", chara, _i);
    }
}

fn main() {
    conditionals();
    count_till_n_for(10);
    count_till_n_while(10);
    iterate_over_str_meth1("hello");
    iterate_over_str_meth2("hello world");
}
