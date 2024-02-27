<img width="645" alt="截屏2024-02-27 22 35 36" src="https://github.com/wycl16514/A_hand_wetted_making_tour_of_rust/assets/7506958/517c515e-97f1-4ec2-84d2-b389847ef687"><img width="962" alt="截屏2024-02-27 22 19 38" src="https://github.com/wycl16514/A_hand_wetted_making_tour_of_rust/assets/7506958/5945f54e-fb45-45d8-b833-3ee63cf7ad69">We have some basics, let's dive directly into projects, by doing lots of projects, we can internalize rust programming skill into ourself.Our first hand wetted project is a simple command line
application. It receives two numbers console which are inputed by user and compute their greatest commond diviser and return the result. There are lots of materials that are talking about 
this algorithm, we won't go into the detail of it, if you are interesting at it, you can info from here: https://en.wikipedia.org/wiki/Greatest_common_divisor

Please make sure you are installed rust on you machine and the cargo command is effective, cd to  an empty directory and using the following command to init a new project:
```r
cargo new gcd
```
then we go into the src forlder and open main.rs for editing, first let's add a function to compute the greatest common diviser:
```r
fn gcd(mut n: i32, mut m:i32) -> i32 {
    //if one of m or n is 0, the program will crash and exit
    assert!(n != 0 && m != 0);
    while m != 0 {
        //exchange the value of m, n if m < n
        if m < n {
            let t = m;
            m = n;
            n = t;
        }

        m %= n;
    }
    //this equal to return n; notices we can not have ; behide n
    n
}
```
in above code, the "-> i32" indicate the function will have a return value with type i32, the assert! marcro verify the code inside it will evalute to true, otherwise it will cause the program
to exit , let's have a try for it, in the main function, we call gcd like this:
```r
fn main() {
    gcd(0, 1);
}
```
and run the command "cargo run" the execute the code, and we will see following:
<img width="823" alt="截屏2024-02-27 22 09 41" src="https://github.com/wycl16514/A_hand_wetted_making_tour_of_rust/assets/7506958/06dc5f96-63c6-48b5-b443-32b4262b15ad">

notices that rust is like golang and python, it dose not require parentheses around the conditional statement for the if or while,and we really need to pay attention is, if you want to
return value from the function, you just place that return value at the end of function and withou semicolon, this is easy to get wrong for beginner, I usally place a semicolon behide it
and cause the function return to fail.

If you are kind to TDD, rust also contain a testing framework, for example we can add test cases like following:
```r
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2*3*5*11*17, 3*7*11*13*19), 3 * 11);
}
```
the "#[test]" is called directive, just like annotation in java or python, function below this diretive will not compiled into the executable binary, but run this testing function using
command "cargo test":
<img width="971" alt="截屏2024-02-27 22 22 46" src="https://github.com/wycl16514/A_hand_wetted_making_tour_of_rust/assets/7506958/0173f66d-e307-493a-8900-e062394344a6">

"cargo test" will execute every testing function in every source file. Diretives in rust just #ifdef, #if..#else in c/c++, they are used to direct rust compiler to do some jobs such as
dismiss warnings or command rust compiler how to iteract with code of other language.

Now let's get the gcd numbers from console, we need to utilize "crat" or packages from rust to help use do the jobs, this is just import of python , js, golang, or include of c/c++.
The package or "crat" we will used is the env under the package named "std", if you are from c/c++, you will find the following code familiar:
```
//env used to parse command line from console
use std::env;

fn main() {
    //arguments from command line comes from env::args()
    let args = env::args();
    for arg in args {
        println!("arg from command line: {}", arg);
    }
}
```

<img width="645" alt="截屏2024-02-27 22 35 36" src="https://github.com/wycl16514/A_hand_wetted_making_tour_of_rust/assets/7506958/e248f7b7-44a4-4326-b9d4-03d2a96b021f">

by default arguments read from command line are strings, if we want to take them as number, we need to convert them from string to number, for example:
```r
//env used to parse command line from console
use std::env;
//FromStr is a trait or interface for converting string into number
use std::str::FromStr;

fn main() {
    let num_str = "123";
    /*
    input for from_str is reference to string, that's why we have a & symbol
    */
    let result = i32::from_str(&num_str);
    /*
    if we want to print a complex type, we need to use{:?}
    */
    println!("value of result is : {:?}", result);
    /*
    result is type of Result, if the convertion is success,
    result would take the value OK that contains the converted value
     */
    match result {
        Ok(val) => {
            println!("the convert value is : {}", val);
        }
        Err(e) => {
            println!("convert is fail for reason: {}", e);
        }
    }
}
```
here comes a intricate concept from rust named trait, you can find equivalence of interface in other language but it is not totally the same, you can see we never call FromStr in the code,
but called from_str from the class i32, there is some mystery relationship between the two, we will spend huge volumn on this topic at later videos, just leave it here.

the "result" has a type of Result, it takes two kind of value, one is Ok, the other is Err, both of them are just like container that contain something inside it, the Ok will contain
the converted number, the Err will contain a error message if the convertion fail.

The match is just like if...else... in other langugae, but it has its own difference, it forces you to handle all possible cases, because the Result type can only have two possible
cases, then we have two branches in {}, if it has three or four possible cases, we need to have three or four branches to handle each possible return otherwise the rust compiler will
give you errors, run the code and we will get the following result:
<img width="1026" alt="截屏2024-02-27 23 13 36" src="https://github.com/wycl16514/A_hand_wetted_making_tour_of_rust/assets/7506958/e4fc6f20-78ed-4ea7-b56d-0f4d25b77d08">

if we change num_str to string other than number, such as  let num_str = "NoN"; then we will get another result:
<img width="1016" alt="截屏2024-02-27 23 17 41" src="https://github.com/wycl16514/A_hand_wetted_making_tour_of_rust/assets/7506958/bb860457-70b0-4935-8c70-6e3a9e546104">

Now let's read a bunch of numbers from command line and call gcd to get their greatest common divisor, let' change the code like this:
```r
fn main() {
    //we use vector to save all arguments from command line
    let mut numbers = Vec::new();
    //get arguments from command line but igonre the first one
    //becuase it is the name of our executbale
    let args = env::args().skip(1);
    for arg in args {
        //:: means class method or static method
        let result = i32::from_str(&arg);
        match result {
            Ok(num) => {
                numbers.push(num);
            }
            Err(e) => {
                //print the error message as err on console
                eprintln!("error parsing arguemnt to number");
                //exit from the app
                std::process::exit(1);
            }
        }
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    //iterate begin from the second element in the vector
    for m in &numbers[1..] {
        //reference and dereference
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

```
In the code, we use vector to save all arguments from command line, the vector is a dynamic array, we can push any number of elements into it.
std::process::exit(1); is exit for error, if the code encounter any unrecoverable error, then we will use it to finish the code immediately.
eprintln! is a macro special for printing out error message, let's run the code by using the command:
```r
cargo run 16 24 32 48 56 
```
and the result is like this:
<img width="1009" alt="截屏2024-02-27 23 41 27" src="https://github.com/wycl16514/A_hand_wetted_making_tour_of_rust/assets/7506958/686fbafd-b8d7-441b-b278-7f703b2763ec">

This is our first hand wetted project, I wish you can have some feeling about rust programming.

