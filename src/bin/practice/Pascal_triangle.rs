fn pascal() {

    const N:usize = 10;
    let mut table = [1; N];
    println!("{}", table[0]);
    println!("{} {}", table[0],table[1]);
    for i in 2.. N {
        let mut j = i -1;
        while j > 0{
            table[j] += table[j-1];
            j -= 1;
        }

        for k in 0..=i {
            print!("{} ", table[k]);
        }
        println!("");


    }
}

fn main(){
    pascal();
}