use std::io;  /*library or module the way in how C language has #include<stdio.h> in place of #include =use and in place of 
<stdio.h>(standard I/O) = std::io*/

fn main() {
  const YEAR:i32=2026; //variable is in case
  let input=String::new(); 
  println!("Enter your birth year:");
  io::stdin()
  .read_line(&mut input)
  .expect("Failed to read line");
  let birthyear:i32=input.trim().parse().expect("Enter a valid number");
  let age:i32=YEAR-birthyear;
  println!("Your age must be:{age}");
  
  
}
