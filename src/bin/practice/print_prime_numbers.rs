
fn is_prime_number(num:i32) -> bool {
    for i in 2..=num {
        if i * i > num {break;}
        if num % i == 0 {return false;}
    }
    true
}

fn print_prime_numbers(num : i32){
    for i in 2..=num{
        if is_prime_number(i) {
            print!("{} ",i);
        }
    }


    println!()
}

fn main(){
    // print_prime_numbers(16); // 2 3 5 7 11 13
    print_prime_numbers(50); // 2 3 5 7 11 13 17 19 23 29 31 37 41 43 47

}