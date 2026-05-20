use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::vec::Vec;
use std::collections::HashMap;

trait Math{
    fn add(&self)->i32;
}
struct num{
    age: i32,
    exp: i32
}

impl Math for num {
    fn add(&self) -> i32 {
        self.age + self.exp
    }
}

// in order to use referneces in a struct we need to tie the lifetime of a struct to the lifetime of the reference defined in it
#[derive(Debug)]
struct User<'a>{
    title: &'a str,
}

fn main() {
    let mut vec=vec![1,2,3,4,5,6,7,8,9]; // a macro for initializing else Vec::new() 
    println!("{:?}",vec);
    println!("The even values of vectors: {:?}", even_filter(&mut vec));
    odd_filter(&mut vec);
    println!("odd values of vectors: {:?}", vec);

    // Hashmap
    let mut map: HashMap<String, u32>=HashMap::new();
    map.insert("Alice".to_string(), 30);
    map.insert("Bob".to_string(), 25);
    println!("HashMap: {:?}", map);  // use .get to get a value form inside
    let user=map.get("Alice");
    println!("Alice's age: {:?}", user); // as you may get a value or nothing the function was implemented with an optional enum
    match user{
        Some(age)=> println!("Alice's age: {}", age),
        None=> println!("Alice's age not found"),
    }

    let mut tl: Vec<(String, i32)>=Vec::new();
    tl.push((String::from("abhinav"), 23));
    tl.push((String::from("sachin"), 25));
    tl.push((String::from("rohit"), 28));
    tl.push((String::from("abhinav"), 26));

    println!("The created hasmap is: {:?}",group_values_by_key_borr(&tl));
    println!("The created hasmap is: {:?}",group_values_by_key(tl));

    // how iterator implement
    let v_real=vec![1,2,3];
    let mut viter=v_real.iter();
    while let Some(val)=viter.next(){ //Will fail when would return none, every iterator implements the following trait under the hood
        println!("The value is: {}", val);
    }


    // iterator adapter
    let a=vec![1,2,3,4,5,6,7,8,9];
    let od_a=a.iter().filter(|x| *x%2!=0).map(|x| (*x)*2);
    print!("The odd values doubled are: ");
    for i in od_a{
        print!("{} ", i);
    }
    println!();

    let name=String::from("abhinav bisht");
    // if to extract the 1st word w euse our noral for method 1) more space used, 2) even if if name is removed, the first name persists! 
    // we want a view of the og string not a new string
    let slice=&name[0..7]; // reference to a part of the word
    // now we cant even clear the the original word, as immutable reference is already taken

    println!("The first name is: {}", slice);

    // you cant return reference to something that was defined within a function as it will be dropped after the function ends, so we can return a reference to a string that was passed as an argument 

    println!("The bigger of the two is: {}",largest(3, 5));

    let to_add: num=num{age: 23, exp: 2};
    println!("age + exp: {}", to_add.add());
    println!("age + exp with implemenation forced parameter: {}", age_exp(to_add));

    // existance of the user should also be tied to the existance of the title, thus we need to setup the relationship bw the lifetime of the struct and the lifetime of the vars defined in it so we need to define the lifetime of the struct and the reference in it, so that they are tied together, thus we can not have a user without a title and also if the title is dropped then the user will also be dropped
    let title=String::from("nitin");
    let nitin=User{title: &title};
    println!("the struct instance is: {:?}", nitin);


    //practicing multi-threading
    thread::spawn(||{
        for i in 1..10{
            println!("number {} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5{
        println!("number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    // the above only prints values from 1-4 on main and 1-7 on spawned threads as the main thread finished execution before the spawned thread causing the spawned thread to be killed before task completion

    println!("Using the handle approach!");

    // to prevent this we use join handle, which causes the main thread to waith for other threads before closing
    let handle = thread::spawn(||{
        for i in 1..10{
        println!("number {} from spawned thread", i);
        thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5{
        println!("number {} from main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
    //  the concepts hold for both thread examples, but now some values from eg1 for oop may seap into the eg2 for loop as the spawned thread isn't killed immediately after the main thread execution as the process is still alive!
    // to validate properly comment out one then test how the execution works for the other! 
    // to validate on a larger scale you can run the 2 for loops to very large values, you will see the values interleaving and even htop will show 200% as both thread are running in parallel

    let vvv=vec![1,2,3];
    let handle=thread::spawn(move||{
        println!("the vector is {:?}", vvv);
    });
    handle.join().unwrap();

    // println!("{:?}",vvv); wont work as the refernece has been moved away from the main thread
    // can retake ownership by, vvv=handle.join().unwrap();

    //how can i borrow, scoped thread guaranteed to finish before scope ends, so borrowed references remain valid
    let v10=vec![1,2,3];
    thread::scope(|s| {
        s.spawn(|| {
            println!("vector with borrowing is: {:?}", v10);
        });
    });
    println!("ownership retained: {:?}", v10);


    // message passing
    let (tx, rx)=mpsc::channel();
        thread::spawn(move||{
        let val=String::from("hi sent from thread");
        tx.send(val).unwrap(); // we need to unwrap as the function return a reslut emum which will return an error if the channel is closed, otherwise where we receive we would have to pattern match          
    });
    let received=rx.recv().unwrap();
    println!("our thread received: {}", received);

    // multiple receivers
    let (tx,rx)=mpsc::channel();

    for i in 0..5{
        let producer=tx.clone();
        thread::spawn(move||{
            let mut ans: u64=0;
            for j in 0..1000{
                ans=ans+(i*1000+j);
            }
            producer.send(ans).unwrap();
        });
    }
    drop(tx); // to close the channel so that the for loop below can end, as all the cloned channels close but the connection to the original tx channel still remains!

    let mut ans: u64=0;
    for val in rx{
        println!("currnet status: {}",ans);
        ans=ans+val;
    }
    println!("Ans is {}", ans);

}

fn even_filter(v: &Vec<i32>)->Vec<i32>{
    let mut new_v=Vec::new();
    for i in v{
        if i%2==0{
            new_v.push(*i);
        }
    }
    return new_v;
}

fn odd_filter(v: &mut Vec<i32>){
    let mut i=0;
    while i < v.len(){
        if v[i]%2==0{
            v.remove(i);
        }else{
            i=i+1;
        }
    }
}

fn group_values_by_key(tl: Vec<(String, i32)>)->HashMap<String, Vec<i32>>{
    let mut map: HashMap<String, Vec<i32>>=HashMap::new();
    for (key, value) in tl{ // not iterating over a vector but over an iterator of vector so we can move the ownership of the value and key to the function
        if map.get(&key).is_none(){
            let mut v: Vec<i32>=Vec::new();
            v.push(value);
            map.insert(key, v);
        }else{
            map.get_mut(&key).unwrap().push(value);
        }
    }
    return map;
}

fn group_values_by_key_borr(tl: &Vec<(String, i32)>)->HashMap<&String, Vec<i32>>{
    let mut map: HashMap<&String, Vec<i32>>=HashMap::new();
    for (key, value) in tl{ // not iterating over a vector but over an iterator of vector so we can move the ownership of the value and key to the function
        if map.get(&key).is_none(){
            let mut v: Vec<i32>=Vec::new();
            v.push(*value);
            map.insert( &key, v);
        }else{
            map.get_mut(&key).unwrap().push(*value);
        }
    }
    return map;
}

// using generics, a and b can be anything as long as they are same and same as the function return type
fn largest<T:std::cmp::PartialOrd>(a:T,b:T)->T{
    if a>b{
        a
    }else{
        b
    }
}

// fn that takes those structs as imput that implement a particular trait
fn age_exp(u: impl Math)->i32{
    u.add()
}