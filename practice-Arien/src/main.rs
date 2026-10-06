use std::collections::HashMap;
/* foo()
{




}

fa()
{




}

*/

/* comments are the same as c
	btw white space doesn't matter (same as c)
*/
//I dont know

fn function(){
	println!("This is a function");

}

fn function2( name: &str ) -> &str {
	println!("This is also a function and you said {}", name);
	return name;
}


fn main(){
        println!("Hello World!");
        print!("This should ");
        println!("be on one line");
        print!("Or one line print macro\n");

//	lets print my name!

	let name = "Arien";
	let age = 19;

	println!("My name is: {} and I'm {}", name, age);

/*
	{} is a place holder to print variables

	let is a keyword that specifies the name 
	of a veriable
*/

//	age = 20;
	//this will not work as variables are immutable?!?!?!
	//but i can make a MUTABLE variable by doing mut
	// keyword
	let mut newage = 19;
	
	println!("I am {}", newage);
	newage = 20;

	println!("But soon i will be {}", newage);

	//THere are 5 main types of variables
	/*int (i32, u32, or f64)
	char
	string (&str)
	bool

	While rust guesses what type you want, you can tell 
	it directly like the following:
	*/


	let fav_num: i32 = 13;
	let fav_double: f64 = 1.3;
	let letter: char = 'A';
	let my_bool: bool = true;
	let string:&str = "ABCD";

	println!("{}, {}, {}, {}, {}", fav_num, fav_double,
	letter, my_bool, string);

	//difference between const and let
	println!("{}",X);
	const X: i32 = 13;
	let xtwo = 13;
	println!("{}",xtwo);

/*	there are 5 differences from the two
	the biggest difference is that constents are 
	compile time variables, meaning you can use them
 	before you declare them
	
	it is proper to have const variables completely 
	uppercase


	feature		constant	variable
	can change	NO		YES with mut
	Type specified	YES		OPTIONAL

*/



//OPERATORS	

	let mut add = 5 + 3;
	let mut sub = 10 - 4;
	let mut mul = 6 * 2;
	let mut div = 12 / 3;
	let mut rem = 10 % 3;

	println!("Answers: {}, {}, {}, {}, {}", add, sub,
	mul, div, rem);

	add += 3;
	sub -= 2;
	mul *= 4;
	div /= 2;
	rem %= 2;

	println!("Answers: {}, {}, {}, {}, {}", add, sub,
	mul, div, rem);


//comparasons

	println!("5 == 10: {}", 5 == 10);
	println!("5 != 10: {}", 5 != 10);
	println!("5 < 10: {}", 5 < 10);
	println!("5 > 10: {}", 5 > 10);


//also have &&, ||, !

//bool

	let boolage = 20;
	let can_vote = boolage >= 18;

	println!("Can vote? {}", can_vote);

	let name = "Arien";

	if name == "Arien" {
		println!("Arien is Awesome");
	}
	else{
		println!("You are awesome");
	}


	let day = 3;

	match day{
	1 => println!("Today"),
	2 => println!("yesterday"),
	3 => println!("Tommorow"),
	_=>println!("Not a day."),
	};

	match day{
	1 | 3 => println!("Day"),
	_ => println!("Not day"),

	};


	let result1 = match day{
		1 | 3 => "Day",
		_ => "Not day",
	};

	println!("{}", result1);


//there is one loop where it repeats forever, you make a 
//statment for it to break.

	let mut count = 1;

	loop{
		println!("loop #{}", count);

		if count == 3 {
			break;
		}

		count += 1;

	}



	count = 1;

	let result = loop{
		println!("Hello");

		if count== 3 {
			break count;
		}
		count += 1;
	};
	println!("The loop stopped at: {}", result);


	count = 1;

	while count <= 5{
		println!("Count: {}", count);
		count += 1;
	}


	function();

	let name: &str = function2("Arien");

	println!("Still {}", name);

/* &str is a non changing string, however String is a
	changing string. This means you can push words
*/

	let mut greeting = String::from("Hellow");
	greeting.push_str(" World");
	println!("{}", greeting);

//or you can concatenate strings

	let s1 = String::from("Hello");
	let s2 = String::from("World!");
	let s3 = String::from("What a great day!");
	let results = format!("{} {} {}", s1, s2, s3);
	println!("{}", results);

	println!("Length: {}", results.len());

//rust uses ownership, if a has a int and you set b to it
// a losses it and b gains it

	let first = String::from("Hello");
	let second = first;

//	println!("{}", first);
	println!("{}", second);

// .clone method fixes it

	let first = second.clone();
	println!("{}", first);
	
//another way to do it is by borrowing by reference

	let a1 = String::from("Hello");
	let b1 = &a1;

	println!("a = {} \nb = {}", a1,b1);

//in order to changea value through reference, reference needs to be mut

	let mut name1 = String::from("Arien");
	let name_ref = &mut name1;
	name_ref.push_str(" Stalford");

	println!("{}", name_ref);

/* rust has arrays, vectors(vec), tuples, hashmaps
*/
//arrays
//you cant add elements to array you can only update existing ones, 
//you can do both in vector
	let fruits = ["apple", "banana", "orange"];
	println!("Last fruit: {}", fruits[2]);


	let mut nums = [1,2,3,4,5,6];
	println!("The first number is: {}", nums[0]);

	nums[0] = 7;
	println!("The new first number is: {}", nums[0]);

	println!("The array has {} elements", nums.len());

	for num in nums{
		println!("{}", num);
	}

	println!("{:?}", nums);




//vectors
	let mut fruitv = vec!["apple", "banana"];
	fruitv.push("orange");

	println!("Last fruit in vec: {}", fruitv[2]);


	fruitv[0] = "grape";

	println!("New first fruit: {}", fruitv[0]);

	fruitv.push("apple");

	println!("{:?}", fruitv);

	//remove last
	fruitv.pop();

	fruitv.insert(0, "cherry");
	println!("{:?}", fruitv);

	fruitv.remove(0);
	println!("{:?}", fruitv);

	println!("There are {} fruits", fruitv.len());

	for fruit in &fruitv{
		println!("{}", fruit);
	}


//tuples
	let person = ("John", 30, true);
	println!("Name : {}", person.0);
	println!("Age: {}", person.1);
	println!("Is active: {}", person.2);


	let (name, age, active) = person;

	println!("Name: {}", name);
	println!("Age: {}", age);
	println!("Active: {}", active);

//hash map uses library in top (use std:: collections::HashMap;)

	let mut capitalcities = HashMap::new();
	capitalcities.insert("France", "Paris");
	capitalcities.insert("Japan", "Tokyo");

	println!("Capital of Japan is {}", capitalcities["Japan"]);



}
