use std::io;

fn main()
{
  let mut input=String::new();
  println!("Enter first number:");
  io::stdin()
  .read_line(&mut input)
  .expect("Invalid input");
  let x:i32=input.trim().parse().expect("Invalid number");
  
  {

  let mut input=String::new();
  println!("Enter second number:");
  io::stdin()
  .read_line(&mut input)
  .expect("Invalid input");
  let y:i32=input.trim().parse().expect("Invalid number");
  
  println!("{x}");
  println!("{y}");
  {
    let mut input=String::new();
  println!("Enter operation:\n1.Addition\n2.Subtraction\n3.Multiplication\n4.Division");
  io::stdin()
  .read_line(&mut input)
  .expect("Invalid input");
   let op:i8=input.trim().parse().expect("Invalid operation");
   match op {
	 1 => println!("= {}",x+y),
         2 => println!("= {}",x-y),
         3 => println!("= {}",x*y),
	 4 => println!("= {}",x/y),
	 _ => println!("Invalid operation"),
     }
  }
  }

}
