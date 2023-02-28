use proconio::input;

fn main() {
    input! {
        (n, k): (i32, i32)
    }
    let mut count = 0;
    for i1 in 1..=n {
        for i2 in 1..=n {
            if i1 + i2 < k && i1 + i2 + n >= k {
                count += 1;
            }
        }
    }
    print!("{}", count);
}
