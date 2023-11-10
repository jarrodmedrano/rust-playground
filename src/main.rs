mod m1_enums;
mod m2_structs;
mod m3_traits;
mod m4_polymorphism;

fn main() {
    // println!("Welcome to this course on {}!@@", OUR_COURSE);

    // let x: i32;
    // x = 2;
    // println!("x = {}", x);

    // let y: i32 = 4;
    // println!("y = {}", y);

    // for i in 0..=y {
    //     if i != 4 {
    //         print!("{}", i);
    //     } else {
    //         println!("{}", i);
    //     }
    // }

    // let mut z: i32 = 5;
    // print!("z was {}", z);
    // z = 10;
    // println!(" and now it is {}", z);

    // let freezing_temp: f64 = -2.4;
    // println!("freezing_temp = {}", freezing_temp);

    // let is_zero_remainder: bool = z % 2 == 0;
    // println!("is_zero_remainder = {}", is_zero_remainder);

    // let my_char: char = 'z';
    // println!("my_char = {}", my_char);

    // let emoji_char: char = '\u{1F600}';
    // println!("emoji_char = {}", emoji_char);

    // let my_floats: [f32; 10] = [0.0; 10];
    // println!("my_floats = {:?}", my_floats);

    // let my_floats_new: [f32; 10] = my_floats.map(|x| x + 1.0);
    // print!("my_floats_new = {:?}", my_floats_new);

    let name: &str = "Rust";
    println!("name = {:?}", name);

    let dynamic_name: String = String::from("Rust");
    println!("dynamic_name = {:?}", dynamic_name);
    println!("my dynamic_name stored in memory {:p}", &dynamic_name);

    let dynamic_name: String = name.to_string();
    println!("dynamic_name = {:?}", dynamic_name);

    let str_slice: &str = &dynamic_name[0..3];
    println!("str_slice = {:?}", str_slice);

    let mut chars: Vec<char> = Vec::new();

    chars.insert(0, 'a');
    println!("chars = {:?}", chars);
    dbg!(&chars);

    let removed_char: char = chars.pop().unwrap();
    println!("removed_char = {:?}", removed_char);

    println!("chars = {:?}", chars);

    chars.insert(0, 'b');

    chars.iter().for_each(|c| println!("c = {}", c));

    let chars_again: Vec<char> = vec!['a', 'b', 'c'];

    let collected: String = chars_again.iter().collect();
    dbg!(collected);
}
