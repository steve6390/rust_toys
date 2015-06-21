
use std::env;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::collections::BTreeMap;

fn main() {
  println!("*** Letter Count ***");
  // skip first arg which is our executable
  for arg in env::args().skip(1) {
    println!("first file is {}", arg);
    let result = File::open(&arg);
    let mut f = match result {
      Err(why) => panic!("Could not open {}: {}", arg, Error::description(&why)),
      Ok(file) => file,
    };
    
    let mut lines = String::new();
    match f.read_to_string(&mut lines) {
      Err(why) => panic!("Could not read {}: {}", arg, Error::description(&why)),
      Ok(count) => println!("Read {} bytes from {}", count, arg),
    }
    
    println!("File {} contains these lines:\n{}", arg, lines);
  
    let mut letter_map = BTreeMap::<char,u32>::new();
  
    for c in lines.chars() {
      *letter_map.entry(c).or_insert(0) += 1;
    }
    
    println!("Letter count for file {}:\n", arg);
    for (c,v) in letter_map {
      println!("{} : {}", c, v);
    }
  }
  
}