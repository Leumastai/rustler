
fn main() {
    let d: i32 = 40;
    changer(&d);
    println!("{}", d);
}


fn changer(b: &i32) {
    // *b += 32;
    // let f: &mut i32 = b;
    // println!("{}\n", f);

    // let f: &i32 = b;
    let g: i32 = *b + 40;
    println!("{}", g);
}