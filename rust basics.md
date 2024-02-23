when you first come to Rust, you will get lots of confusion. For other programming languages they are quit similar to each other, but Rust is different, the differencies between Rust and other languages are innovative of Rust, The downsize is that the learning curve of Rust is quit steep compare with other language.The first we need to get used to is the transfer of ownship for variables, this is never seen for other languages, Let's have a try, use cargo new RustBasic to create a new project.

The first thing we need to know is , variables in rust are const by default, which means you can't change its init value, for example in the following code, the rust compiler will give your complain and refuse to compile your code:

<img width="827" alt="截屏2024-02-22 22 50 06" src="https://github.com/wycl16514/A_hand_wetted_making_tour_of_rust/assets/7506958/05dd9979-f863-4c48-9b10-ff1cac52e648">

if you want to reassign value to variable, you need to add the "mut" keyword, just like following:
```r
fn main() {
    let mut a = 1;
    a = 2;
    println!("a is {}", a);
}
```
and you run command "cargo run", you can get the result. The second point is transfer of owership,you will get confused for the following code if you have experiences for other programming language:
```r
struct User {
    name: String,
}

fn print_user(user: User) {
    println!("name of user is {}", user.name);
}
fn main() {
    let user = User {
        name: "Tylor".to_string(),
    };
    print_user(user);
    //user can't be used again, because ownership is transfer to print_user
    print!("user name: {}", user.name);
}
```
if you compile above code , you will get the following error:
<img width="1115" alt="截屏2024-02-22 23 08 23" src="https://github.com/wycl16514/A_hand_wetted_making_tour_of_rust/assets/7506958/79c1752e-fc38-4af9-8818-654516329a35">

but the transfer of ownership will not happend on basic type such as u64, or i32, the following code is ok, very confusing right:
```r
fn print_val(a: u64) {
    println!("value of a is :{}", a);
}
fn main() {
    let a = 2;
    print_val(a);
    //you can still manuplate a here:
    println!("value of a is : {}", a);
}
```
if you get confused, don't worry, we will get clarify in later chapters.How can we keep the ownership of User object? we need to use reference, the following code will avoid the problem happened above:
```r
struct User {
    name: String,
}

fn print_user(user: &User) {
    //& means reference and won't request the ownership
    println!("name of user is {}", user.name);
}

fn main() {
    let user = User {
        name: "chenyi".to_string(),
    };
    print_user(&user);
    println!("name of user : {}", user.name);
}
```
the symbol '&' means references, if you come from c++ you will familiar with it. It tell the compiler that the function will only borrow the input object and keep the ownership for
the caller, and that's why when we finish calling print_user and we can still use the User instance in the main function. Finnally let's see reference and dereference, see the followint code: 
```r
fn main() {
    let mut x = 1;
    let x_ref = &mut x;
    /*
    x_ref is a mutable
    x_ref is a container of x, *x_ref is get x out from the container of x
     */
    *x_ref += 1;
    //what is the value of x?
    println!("x is: {}", x);
}

```
x_ref is a mutable reference to x, actually is a x_ref is a container for x, the symbol '*' is called dereference, which means get the object contained in the container out from the container, which means *x_ref is the same as the variable x, run the code and you will see the value of x turn into 2, that's all for the basics of rust, let's go to a real project of rust.
