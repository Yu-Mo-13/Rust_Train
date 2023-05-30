fn main() {
    let mut var1: i32 = 6;
    let mut var2: i32 = 7;

    while var1 * var2 > 0 {
        // var1とvar2を1ずつ減らす
        var1 -= 1;
        var2 -= 1;
        println!("var1={}, var2={}", var1, var2);
        println!("Hello, world!");
    }
}
