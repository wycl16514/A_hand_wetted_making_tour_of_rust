fn gcd(mut n: i32, mut m: i32) -> i32 {
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

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

//env used to parse command line from console
use std::env;
//FromStr is a trait or interface for converting string into number
use std::str::FromStr;

// fn main() {
//     let num_str = "123";
//     //let num_str = "NoN";
//     //if the string is not a number, then the code print out string in expect
//     //and exit
//     let result = i32::from_str(&num_str);
//     /*
//     if we want to print a complex type, we need to use{:?}
//     */
//     println!("value of result is : {:?}", result);
//     /*
//     result is type of Result, if the convertion is success,
//     result would take the value OK that contains the converted value
//      */
//     match result {
//         Ok(val) => {
//             println!("the convert value is : {}", val);
//         }
//         Err(e) => {
//             println!("convert is fail for reason: {}", e);
//         }
//     }
// }

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
