use proconio::input;

fn main() {
    input! {
        (n, x):(i16, i16),
        data:[i16;n],
    }
    let result = data.into_iter().find(|a| a == &x);
    if let Some(_) = result {
        println!("Yes");
    } else {
        println!("No");
    }
}
