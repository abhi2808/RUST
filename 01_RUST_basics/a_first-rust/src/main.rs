use std::fmt::format;
use std::fmt::Debug;
use std::io::Error;
use std::fs;


struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
impl Debug for User{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "User {{ active: {}, username: {}, email: {}, sign_in_count: {} }}", self.active, self.username, self.email, self.sign_in_count);
    }
}
    

//implementing structs
struct Rect{
    w: u16,
    h: u16,
}
impl Rect{
    fn area(&self)->u16{
        return self.w*self.h;
    }
}

#[derive(Debug)]
enum Direction{
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
enum Shape{
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64)
}


fn main() {
    let x: i32=-1; // i=>signed integer, u=>unsigned integer, eg i8, i16, i32! max no: 2^n -1 for i and 2^n for u
    let mut y: u8=2; // you cannot modify a var in rust unless its declared mutable 
    let z: f32=3.0;

    // while asigning a value to a variable if you overflow, an error occurs, but if its value is incremented using runtime logic then no error(but runtime) as the compiler doen't run your code only static analysis
    for _i in 0..5{
        y=y+1; // if y+100 then runtime error
    }

    println!("x:{}, y:{}, z:{}", x, y, z);


    let is_male=false;
    let is_above_18=false;
    if is_male{
        if is_above_18{
            println!("You are an adult man");
        }
    }
    else if !is_male && !is_above_18{
        println!("You are neither an adult nor a man");
    }


    // if variable can change its space during runtime then creating is tricky eg string arrays etc
    let greeting1: String=String::from("hello world");
    let _greeting2: &str = "hello world";
    let char1: Option<char> = greeting1.chars().nth(1000); // as the element at the 1000th index may not necessarily be a character

    match char1{
        Some(c) => println!("{}",c),
        None => println!("No character at this index"),
    }

    // println!("char is {}", char1.unwrap()); // usning unwrap tells we are okay with the exception
    println!("char is {:?}", char1);

    let i: u8= 10;
    if i<=10 && i!=4 {
        println!("learnt conditionals")
    }

    let sentence: String=String::from("my name is abhinav");
    let firstWrd: String=get1stWrd(sentence);
    println!("first word is: {}", firstWrd);

    let s1:String=String::from("coders, ");
    let s2:String=String::from("code");
    let combined:String=format!("{}{}",s1,s2);
    println!("combined: {}",combined);
    println!("string working, capacity: {}, length: {}, pointer: {:p}", combined.capacity(),combined.len(), combined.as_ptr());

    // if you are not using something that you created the prefix using _, eg _i

    let str1: String=String::from("Hello");
    print!("str1: {}, ", str1);
    let str2: String=str1; 
    // print!("str1: {}, ", str1);  gives error
    println!("str1: {}, ", str2);


    let my_s=String::from("my string");
    // take_ownership(my_s);
    // println!("my_s: {}", my_s); // gives error as ownership of my_s is taken by the function and it is dropped after the function call
    take_ownership(my_s.clone());
    println!("my_s: {}", my_s);

    // such a scenario wont happen if the function returns a value then the ownership is taken back to the original one
    let mut ms:String=String::from("my string");
    ms=take_ownership_and_return(ms);
    println!("ms: {}", ms);

    
    let mut im_s: String=String::from("borrowing");
    imut_borrow(&im_s);
    println!("unchanged: {}", im_s); // as the ownership is not taken

    mut_borrow(&mut im_s);
    println!("changed: {}", im_s); // as the ownership is not taken but the

    let abhinav: User=User {
        active: true,
        username: String::from("abhinav"),
        email: String::from("abhinav@gmail.com"),
        sign_in_count: 1,
    };
    println!("User details: {:?}", abhinav);
    // other types are unit structs and tuple structs, unit structs basically without attributes used like classes without attributes and tuple structs are basically tuples with named struct

    let my_rect: Rect=Rect{
        w: 10,
        h: 20,
    };
    println!("area of rectangle: {}", my_rect.area());


    // enums: get valuse from a set of possible values
    move_around(&Direction::East);

    //pattern matching with enums
    let my_shape: Shape=Shape::Square(10.0);
    println!("area of {:?}: {}", my_shape, caculate_area(&my_shape));


    // result enums: used for error handling, basically an enum with two variants Ok and Err, Ok is used when the function executes successfully and Err is used when there is an error, both variants can hold a value, Ok holds the return value of the function and Err holds the error message
    let res: Result<String, Error> = fs::read_to_string("example.txt");
    match res{
        Ok(content)=>{
            println!("File content: {}", content);
        },
        Err(err)=>{
            println!("Error reading file: {}", err);
        }
    }

}

fn get1stWrd(sentence: String)->String{
    let mut ans: String=String::new();
    for char in sentence.chars(){
        ans.push(char);
        if char==' '{
            break;
        }
    }
    return ans;
}

fn take_ownership(s: String){
    println!("ownership of {} taken",s);
}

fn take_ownership_and_return(s: String)->String{
    println!("ownership of {} taken temperorily",s);
    return s;
}

fn imut_borrow(s: &String){
    println!("borrowed immutably: {}", s);
}

fn mut_borrow(s: &mut String){
    s.push_str(" made mutable");
    println!("borrowed mutably: {}", s);
}

fn move_around(d: &Direction){
    println!("moving to: {:?}",d);
}

fn caculate_area(shape: &Shape)->f64{
    match shape{
        Shape::Circle(r) => 3.14*r*r,
        Shape::Square(s) => s*s,
        Shape::Rectangle(l, b) => l*b,
    }
}