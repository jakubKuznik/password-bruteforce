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

//use std::net::TcpStream;
//use ssh2::Session;


fn main() {

  // Char possibilities: var holding input params. 
  //  (-n,-c,-C,-s)
  //  argument parsing 
  let mut charPos = argParse(); 
  println!("{} {} {} {} ", charPos.0, charPos.1, charPos.2, charPos.3);
   
  if   charPos.0 == false && charPos.1 == false 
    && charPos.2 == false && charPos.3 == false {
    println!("kkt");
  }

 
  generatePasswords(charPos);
  
//  let tcp = TcpStream::connect("127.0.0.1:22").unwrap();
//  let mut sess = Session::new().unwrap();
//  sess.set_tcp_stream(tcp);
//  sess.handshak().unwrap();
//  sess.userauth_password("jakub", "hovno").unwrap();
//  assert!(sess.authenticated());



}

// generate passwords and print them to the stdout 
fn generatePasswords((n, c, C, s): (bool, bool, bool, bool) ) { 
  
  // s .. ASCII 32 .. 47 + 58 .. 64 + 91 .. 96 + 123 .. 126
  // C .. ASCII 65 .. 90
  // c .. ASCII 97 .. 122
  // n .. ASCII 30 .. 39

  println!("{} {} {} {} ", n, c, C, s);

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


