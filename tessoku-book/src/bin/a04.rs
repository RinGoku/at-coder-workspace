use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let result: Vec<String> = (0..10).map(|i| ((n / (2_i32.pow(i)) as usize) % 2).to_string() ).collect();
    println!("{}", result.concat().chars().rev().collect::<String>())
}

