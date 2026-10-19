fn main() {
  let mut x:i8=0;

  loop {
	  match x {
	    0 => println!("Wrong guess"),
	    1 => println!("Wrong guess"),
	    2 => println!("Wrong guess"),
	    3 => break,
            _ => println!("You lost"),
	   }
	   x+=1;
	    
	  }
	}
