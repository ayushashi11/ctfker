use std::env;
fn main(){
   let args = env::args();
   if args.len()<=1{
      println!(r#"
Usage: deployer PATH PORT
PORT is ignored for web programs
      "#);
   }
}