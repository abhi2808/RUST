use chrono::{Local ,Utc};

enum Shape{
    Rectangle(f64,f64),
    Circle(f64),
}

struct people{ // more closer ot classes than objects
    name: String,
    age: i32,
}
impl people{
    fn hurray(&self){
        println!("Hurray {}, you did a great job!", self.name);
    }
    fn general(){
        println!("people are good!");
    }
}

fn main() {
    let num=2;
    is_even(num);
    fib(num);
    let s: String=String::from("I am abhinav");
    println!("The length of string {} is {}", s, get_len(&s));
    let abhinav = people{
        name: String::from("Abhinav"),
        age: 23,
    };println!("The age of {} is {}", abhinav.name, abhinav.age); // the println macros is able to print these but would error out for abhinav as it doesn't know how to print that, then we would need to use a debug trait! 
    abhinav.hurray(); //calling the implemented function for the struct!
    // if you are passing other parameters apart from self then you need to specify the type of that parameter in the function definition and then pass it while calling the function
    // abhinav.general(); would cause an error as not using self, but can be as a static function of the striuct!
    people::general();

    // enums help us enumerate over various types of a value, good for type safety if the possible value of a vaiable can only be 1 out of a fixed set!
    let rectangle = Shape::Rectangle(10.0, 20.0);
    let circle = Shape::Circle(5.0);
    println!("The area of the rectangle is {}", calculate_area(rectangle));
    println!("The area of the circle is {}", calculate_area(circle));

    let str:String=String::from("if you good, find here");
    // println!("what was returned: {:?}",first_a(str));
    let res=first_a(str);
    match res{
        Some(value)=> println!("a was found 0/1: {}", value),
        None=> println!("char a not found")
    }
    
    // Result enum
    let file_content=std::fs::read_to_string("hello.txt");
    match file_content{
        Ok(file_content)=>{
            println!("File content: {}", file_content);
        }
        Err(error)=>{
            println!("Error reading file: {}", error);
        }
    }
    
    let path=String::from("haha.txt");
    println!("gosh this sucks: {:?}",dummy_fun(path));

    println!("the time now is {}", Local::now());


}

fn is_even(num: i32){
    if num%2==0{
        println!("{} is even",num);
        return;
    }
    println!("{} is odd", num); // macro printing dynamic variable
}

fn fib(num: i32){
    let mut a = 0;
    let mut b= 1;
    for _ in 0..num{
        let temp = b;
        b=a+b;
        a=temp;
    }
    println!("The {}th Fibonacci number is {}", num, a);
}

fn get_len(s: &String)-> usize{
    s.chars().count() // you can return like this but remember not to use a semi-colon
}

fn calculate_area(shape:Shape)->f64{
    let area=match shape{
        Shape::Rectangle(a,b)=> a*b as f64,
        Shape::Circle(r)=> std::f64::consts::PI*r*r,
    };
    area
}

fn first_a(s: String)->Option<i32>{
    let mut i:i32=0;
    for char in s.chars(){
        if char=='a'{
            println!("first a found and at index {}", i);
            return Some(i);  // if defined your own enum say customEnum then customEnum::Some(i) and customEnum::None, the real one implemented uses generics
        }
        i=i+1;
    }
    return None;
}

fn dummy_fun(path: String)->Result<String,String>{
    let res=std::fs::read_to_string(path);
    match res{
        Ok(data)=> Ok(String::from("all good")),
        Err(error)=> Err(String::from("errors kill me each day")),
    }
}