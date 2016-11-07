// this main is an example of prinling, variables, mutability
// fn main() {
//     println!("New things!");
//     println!("Two things!");
//     let name = "katrina";
//     let age = 29;
//     println!("Hi {}! You are {} years old!", name, age);
//     let mut apples = 100;
//     apples += 50;
//     println!("I have {} apples", apples);
// }

// this is an example of how functions work; it's called below in main
// fn add_fifty(n: i32) -> i32 {
//     n + 50
// }

// fn main() {
//     println!("Lots: {}", add_fifty(100));
// }

// this function is CONDITIONAL SWITCH
// fn main() {
//     let age = 50u32;
//     match age {
//         0...12 => println!("You may see G or PG movies."),
//         13...16 => println!("You may see G, PG, or PG-13 movies."),
//         // _ is the "catch all"
//         _ => {} // this means do nothing
//         // _ => {
//         //     println!("You are old.");
//         //     println!("You may see G, PG, or PG-13 movies")
//         // },
//     }
// }

// this function is CONDITIONAL IF
// fn main2() {
//     let n = 13u32;
//     if n < 13 {
//         println!("You may see G or PG movies");
//     } else if n < 17 {
//         println!("You may see G, PG, or PG-13 movies");
//     } else {
//         println!("You are old.");
//         println!("You may see G, PG, PG-13 or R movies.")
//     }
// }

// arrays
// fn main(){
//     let color = [255, 0, 255];
//     let index = 9;
//     println!("The color is {:#?}", color[index]);
// }

// vectors
// fn main(){
//     let mut prices = vec![30, 100, 2];
//     prices[0] = 25;
//     prices.push(40);
//     println!("All the prices are: {:?}", prices);
// }

// for loop
// fn main(){
// for i in 0..10 {
//     println!("Number {}", i);
//     }
// }

// for loop iterating over a vector
// fn main(){
//     let names = vec!["Carol", "Jake", "Marylou", "Bruce"];
//     for name in names.iter() {
//       println!("Hi {}!", name);
//     }
// }

// iterators
// fn main() {
//     for i in (0..10).filter(|x| x % 2 == 0) {
//         println!("i = {}", i);
//     }
//     for i in (0..10).map(|x| x * x ) {
//         println!("i = {}", i);
//     }
//     let sum = (0..10).fold(0, |acc, x| acc + x);
//     println!("sum = {}", sum);
// }

// enums
// fn main() {
//     enum TrafficLight {
//         Red,
//         Yellow,
//         Green,
//     }
//     let light = TrafficLight::Green;
//     match light {
//         TrafficLight::Red => println!("STOP!"),
//         TrafficLight::Yellow => println!("Slow down!"),
//         TrafficLight::Green => println!("Go go go!"),
//         // or
//         // TrafficLight::Green | TrafficLight::Yellow => println!("Go go go!"),
//     }
// }


// enums can hold values
// fn main() {
//     enum GameType {
//         SinglePlayer,
//         MultiPlayer(u32),
//     }
//     let game = GameType::MultiPlayer(50);
//     match game {
//         GameType::SinglePlayer => println!("How about solitaire?"),
//         GameType::MultiPlayer(2) => println!("How about checkers?"),
//         GameType::MultiPlayer(4) => println!("How about bridge?"),
//         // this num lets you get the variable out of the enum variant
//         GameType::MultiPlayer(num) => {
//             println!("How about {}-player tag?", num)
//         },
//     }
// }

// more enums!
// fn main() {
//     enum GameType {
//         SinglePlayer,
//         MultiPlayer(u32),
//     }
//     let game = GameType::MultiPlayer(50);
//     let two = 2;
//     match game {
//         GameType::SinglePlayer => println!("How about solitaire?"),
//         // this two is like the num we used in the last case before;
            // it will match everything
//         // so you'll never to to the following to cases
//         // also, this is scoped to the match and is not the same as the two = 2 above
//         GameType::MultiPlayer(two) => println!("How about checkers?"),
//         GameType::MultiPlayer(4) => println!("How about bridge?"),
//         // this num lets you get the variable out of the enum variant
//         // GameType::MultiPlayer(num) => {
//         //     println!("How about {}-player tag?", num)
//         // },
//         GameType::MultiPlayer(_) => {
//             println!("How about tag?")
//         },
//     }
// }


// fn main() {
//     let b: Option<&str> = None;
//     match b {
//         Some(name) => {
//             println!("Other name is {} bytes long", name.len())
//         },
//         None => {
//             println!("No name!")
//         }
//     }
// }

// fn main() {
//     let numstr = "6";
//     let num = numstr.parse::<i32>();
//     println!("num = {:?}", num);
//     // prints num = Ok(6)
//
//     let numstr = "florp";
//     let num = numstr.parse::<i32>();
//     println!("num = {:?}", num);
//     // prints num = Err(ParseIntError { kind: InvalidDigit }
//
//     // example with match
//     let numstr = "florp";
//     let num = numstr.parse::<i32>();
//     let answer = match num {
//         Ok(n) => n + 5,
//         Err(_) => 0,
//     };
//     println!("Answer is {}", answer);
//     // prints Answer is 0
//
//     // using with expect
//     let numstr = "6";
//     let num = numstr.parse::<i32>();
//     let num = num.expect("should have a number");
//     println!("num + 5 = {}", num + 5);
//     // prints num + 5 = 11
// }

// try
// fn add_five_to_string(s: String) ->
//     Result<i32, std::num::ParseIntError> {
//
//         match s.parse::<i32>() {
//                Ok(val) => Ok(val + 5),
//                Err(e) => Err(e),
//            }
//         // let ans = try!(s.parse::<i32>()) + 5;
//         // Ok(ans)
// }
//
// fn main() {
//     println!("{:?}", add_five_to_string("50".to_string()));
//     println!("{:?}", add_five_to_string("this".to_string()));
// }

// strings are weird!
// fn fizz(num: u32) -> String {
//     if num % 3 == 0 {
//         // this doesn't work
//         // "Fizz"
//         // but this does
//         "Fizz".to_string()
//     } else {
//         num.to_string()
//     }
// }
//
// fn main() {
//     println!("{}", fizz(3));
// }
// fn main() {
//     let v = vec![1, 2, 3, 4, 5];
//     let piece = &v[..];
//     println!("piece of v = {:?}", piece);
// }

// fn main() {
//     let s = String::from("Call me Ishamel blah blah ...");
//     let part = &s[0..4];
//
//     println!("part is '{}'", part);
// }

// fn main() {
//      let v = vec![1, 2, 3,];
//      println!("v is valid here! {:?}", v);
// }

// fn main() {
//     let v = vec![1, 2, 3];
//     // the function now returns a value, so we take the value
        // and pass it to the next function
//     let w = print_vec(v);
//     print_vec(w);
// }
// fn print_vec(v: Vec<i32>) -> Vec<i32> { // -> Vec<i32> returns the vector,
//              allowing us to do the above
//     println!("v is {:?}", v);
//     v
// }

// fn main() {
//     let v = vec![1, 2, 3];
//     print_vec(&v[..]);
//     print_vec(&v[..]);
// }
// fn print_vec(v: &[i32]) {
//     println!("v is {:?}", v);
// }

// fn main() {
//     let mut v = vec![1, 2, 3];
//     change_vec(&mut v[..]);
//     change_vec(&mut v[..]);
//     println!("v is {:?}", v);
// }
// fn change_vec(v: &mut [i32]) {
//     v[0] *= 5;
// }

// fn main() {
//     let mut v = vec![1, 2, 3];
//     let f = &v[0];
//     v.clear();
//     println!("What would f be? {}", f);
// }
