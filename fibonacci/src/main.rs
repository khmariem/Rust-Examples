fn main() {
    let n: u32 = 10;
    let mut n0:u32 = 0;
    let mut n1:u32 = 1;
    let mut n2:u32 = 0;

    for _i in 2..n+1 {
        n2 = n0 + n1;
        n0 = n1;
        n1 = n2;
    }

    println!("The {}th fibonacci number is {}", n, n2);
}
