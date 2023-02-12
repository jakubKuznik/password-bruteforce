// Authors: Jakub Kuzník <xkuzni04>
// Description: 
//  print all possible passwords to stdout separated by \n. 
//
// build:
//  rustc generate.rs
//
// exectuion: 
//  ./generate [-n number] [-C Capitals] [-c characters] [-s special]
//
// params 
//  -n == 0-9 
//  -C == A-Z
//  -c == a-z 
//  -s == / * + & ...
//

use std::env;


fn main() {

  // Char possibilities: var holding input params. 
  //  (-n,-c,-C,-s)
  //  argument parsing 
  let mut charPos = argParse(); 

  println!("{} {} {} {} ", charPos.0, charPos.1, charPos.2, charPos.3);

}

fn argParse() -> (bool, bool, bool, bool){
  
  let mut n = false; // -n
  let mut C = false; // -C
  let mut c = false; // -c
  let mut s = false; // -s
  
  println!("hi");
  
  let args: Vec<String> = env::args().collect();

  for i in 0..args.len() {
    if args[i] == "-n"{
      n = true;
    }
    else if args[i] == "-c"{
      c = true;
    }  
    else if args[i] == "-C"{
      C = true;
    }  
    else if args[i] == "-s"{
      s = true;
    }  
    println!("{} {}", args.len(), args[i]);
  }
  return (n, c, C, s);
}

