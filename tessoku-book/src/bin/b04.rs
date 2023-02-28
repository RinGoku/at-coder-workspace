use proconio::input;

fn main() {
    input! { 
        n: usize
    }
    let n_str = n.to_string();
    let rev_n_str = n_str.chars().rev().collect::<String>();
    let result = rev_n_str.chars().enumerate().fold(0, |a, (i, c)| a + c.to_string().parse::<usize>().unwrap() * (2_i32.pow(i as u32) as usize ));
    println!("{}", result);
}
