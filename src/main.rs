mod calculator;

fn main() {
  let result = calculator::Calculator::parse("2 * 2 + 48 / 4");
  match result {
    Ok(tokens) => {
      println!("{:?}", tokens);
      let expr = calculator::Calculator::expression(tokens);
      println!("{:?}", expr);
    }
    _ => println!("Parsing error!")
  }
}
