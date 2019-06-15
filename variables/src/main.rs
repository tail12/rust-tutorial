fn main() {
    // シャドーイングの例
    let spaces = "    ";
    let spaces = spaces.len();
    println!("space length: {}", spaces);

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("THe value of x is: {}", x);

    // data struct
    let guess: u32 = "42".parse()
        .expect("Not a number!");

    // float
    let fx = 2.0;
    let fy: f32 = 3.0;

    let tup = ("hoge", 1, 2.1);
    let (str_hoge, one, two_point_one) = tup;
    println!("{}", str_hoge)
}
