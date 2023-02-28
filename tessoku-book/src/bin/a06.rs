use proconio::input;

fn main() {
    input! {
        (n, q): (u32, u32),
        ass: [u32;n],
        ls: [(u32, u32 );q]
    }
    let mut summed =vec![0;ass.len() + 1];
    summed[0] = 0;
    for (i, a) in ass.iter().enumerate() {
        summed[i + 1] = summed[i] + a;
    }
    for (l, r) in ls.iter() {
        println!("{}", summed[*r as usize] - summed[*l as usize - 1]);
    }
}
