use std::io;

use calculator::{Calculator, Error};
mod calculator;


fn main() -> Result<(), Error> {
  // let result = calculator::Calculator::parse("2 * 2 + 48 / 4");
  // match result {
  //   Ok(tokens) => {
  //     println!("{:?}", tokens);
  //     let expr = calculator::Calculator::expression(tokens);
  //     println!("{:?}", expr);
  //     let result = calculator::Calculator::evaluate(expr);
  //     match result {
  //       Some(value) => println!("Value of result is: {}", value),
  //       None => println!("Somthing wrong you!")
  //     };
  //   }
  //   _ => println!("Parsing error!")
  // }

  loop {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
      Ok(_) => {
        let tokens = calculator::Calculator::parse(input);
        if tokens.is_err() {
          println!("{:?}", tokens.err().unwrap());
          continue;
        }
        let expr = calculator::Calculator::expression(tokens?);
        if let Some(value) = calculator::Calculator::evaluate(expr) {
          println!("{}", value);
        }
      },
      Err(error) => println!("error: {}", error)
    }
  }
}
