fn print_multiplication_table() {

    println!("  |  1  2  3  4  5  6  7  8  9");
    println!("--+---------------------------");
    for i in 1..10 {
        print!("{} | ", i);
        for j in 1..10{
            // 2桁表示
            print!("{:2} ", i * j);
        }
        println!("");

    }
}



fn main() {
    print_multiplication_table();
}