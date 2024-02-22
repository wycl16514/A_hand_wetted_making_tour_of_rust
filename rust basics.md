when you first come to Rust, you will get lots of confusion. For other programming languages they are quit similar to each other, but Rust is different, the differencies between Rust and other languages are innovative of Rust, The downsize is that the learning curve of Rust is quit steep compare with other language.The first we need to get used to is the transfer of ownship for variables, this is never seen for other languages, Let's have a try, use cargo new RustBasic to create a new project.

The first thing we need to know is , variables in rust are const by default, which means you can't change its init value, for example in the following code, the rust compiler will give your complain and refuse to compile your code:

<img width="827" alt="截屏2024-02-22 22 50 06" src="https://github.com/wycl16514/A_hand_wetted_making_tour_of_rust/assets/7506958/05dd9979-f863-4c48-9b10-ff1cac52e648">

if you want to reassign value to variable, you need to add the "mut" keyword, just like following:
'''rust
fn main() {
    let mut a = 1;
    a = 2;
    println!("a is {}", a);
}
'''
