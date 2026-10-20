use std::io;
fn main()
{
  println!("What's your gender?");
  let mut input=String::new();
  io::stdin()
  .read_line(&mut input)
  .expect("Invalid input");
  let gender:&str=input.trim();
   match gender {
   "M" => println!("You're a male!"),
   "F" => println!("You're a female!"),
    _ => println!("Enter Valid Input!"),
   }
}
