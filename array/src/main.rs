fn main() {
    let arr: [u32; 11] = [45,3,56,234,0,1,5543,76,3456,4,13];
    println!("Original array {:?}", arr);
    println!("We are going to make some calculations:");
    println!("Max...");
    println!("{}",maxme(arr));
    println!("Min...");
    println!("{}",minme(arr));
    println!("Sum...");
    println!("{}",summe(arr));
    let x:u32 = 234;
    println!("Find {} in the array...",x);
    let res:u32 = findme(arr, x);
    if res == 0 {
        println!("Not found.");
    }
    else {
        println!("Found at {}.",res-1);
    }
}

fn maxme(arr: [u32;11]) -> u32 {
    let mut m:u32 = 0;
    for n in 0..arr.len() {
        if arr[n]>m {
            m = arr[n];
        }
    }
    m

}

fn minme(arr: [u32;11]) -> u32 {
    let mut m:u32 = 10000;
    for n in 0..arr.len() {
        if arr[n]<m {
            m = arr[n];
        }
    }
    m
}

fn summe(arr: [u32;11]) -> u32 {
    let mut m:u32 = 0;
    for n in 0..arr.len() {
        m = m+arr[n];
    }
    m
}

fn findme(arr: [u32;11], x:u32) -> u32 {
    let mut n:usize = 0;
    let mut n1 = 0;
    let m:u32 = loop {       
        if arr[n]==x {
            break n1+1;
        }

        if n==arr.len()-1{
            break 0;
        }
        n = n+1;
        n1 = n1+1;
    };

    m
}