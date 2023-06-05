fn main() {
    let mut var1: i32 = 6;
    let mut var2: i32 = 7;
    let mut var3: i32;

    while var1 * var2 > 0 {
        var1 = multiple(var2) - var1;
        var3 = shrink(var1, var2);
        var2 = var1 - var3;
        println!("var1={}, var2={}, var3={}", var1, var2, var3);
        println!("Hello, world!");
    }
}

fn multiple(x: i32) -> i32 {
    if x < 0 {
        return 0;
    }
    return x * x;
}

fn shrink(x: i32, y: i32) -> i32 {
    let len: i32 = 10 * y;
    if x < 0 {
        return 0;
    }
    return x / len;
}
