struct Rect {
    width: u32,
    height: u32
}

fn main() {
    let rect1 = Rect{
        width:10,
        height:5
    };
    println!("Area for a rectangle of width {} and height {} is: {}", rect1.width, rect1.height, rect1.width*rect1.height);
}
