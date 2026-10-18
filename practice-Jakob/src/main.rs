fn main() {



    /*const NUM_T: u32 = 5;
    println!("Hello, world!");
    print!("on the same line \n woah");
    println!("Hey guys");

    let name: &str = "j";
    let age: i32 = -3;
    println!("My name is {}, my age is {}", name, age);


    let x = 8;
    let oldEnough =   x > 21;
    println!("{}", oldEnough);

    let mut  x = 5;
    x *= 3;

    if x < 5
    {
        println!("{}", x);
    }
    else if x > 5
    {
        println!("nah");
    }

    let dog = "oreo";
    let greeting = match dog {
        "oreo" => "Hi oreo!",
        "lucky" => "You don't exist",
        _ => "You are a fake dog"
    };
    println!("Hi, {}", greeting);

    for i in 1..=5{
        let printer = i * 5;
        if i == 2{
            continue;
        }
        println!("{}",printer);
    }*/
    
    let mut text: String = "Hello".to_string();
    new(&mut text);
    println!("{}", text);
    
}

fn new(fun_text: &mut String) {
    let newText: String = " world!".to_string();
    fun_text.push_str(&newText);
}
