#![allow(warnings)]

fn main(){
  for x in 1..100 {
    let mut res: String = "".to_owned(); // what the fuck

    if (x % 3 == 0) {
      res += "Fizz";
    }
    if (x % 5 == 0) {
      res += "Buzz";
    }

    if res == "" {
      res = x.to_string()
    }

    println!("{}", res);
  }
}
