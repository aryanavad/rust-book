fn main() {
    let n = 5;
    let mut n1 = 0;
    let mut n2 = 1; 
    let mut ans = 0;
    for _ in 0..n {
        ans = n1 + n2;
        n1 = n2;
        n2 = ans;
    }
    println!("digit at {n} place: {ans}");
}
