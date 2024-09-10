#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
  Add,
  Sub,
  Mul,
  Div,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
  Number(u32),
  Op(Operator),
  Bracket(char),
}

pub struct Calculator {}

#[derive(Debug)]
pub enum Error {
  BadToken(char),
  MismatchedParens,
}


impl Calculator {
  pub fn parse<T: AsRef<str>>(expr: T) -> Result<Vec<Token>, Error> {
    let expr = expr.as_ref();
    let chars = expr.chars();
    let mut tokens: Vec<Token> = Vec::new();
    let mut parens = Vec::new();
    for c in chars {
      match c {
        '0'..='9' => match tokens.last_mut() {
          Some(Token::Number(n)) => {
            *n = *n * 10 + ((c as u32) - 48);
          }
          _ => {
            let digit = (c as u32) - 48;
            tokens.push(Token::Number(digit));
          }
        },
        '(' => {
          tokens.push(Token::Bracket('(')); // replace to `c` var;
          parens.push(c);
        }
        ')' => {
          tokens.push(Token::Bracket(')'));
          if let Some(p) = parens.pop() {
            if p != '(' {
              // Nothing put in this parenthesis `()`,
              // so, we return an error, in place without
              // continue.
              return Err(Error::MismatchedParens);
            }
          } else {
            // Nothing parenthsis open, so we cannot close
            // parenthesis in this case. We return an error
            // in place.
            return Err(Error::MismatchedParens);
          }
        }
        '+' => tokens.push(Token::Op(Operator::Add)),
        '-' => tokens.push(Token::Op(Operator::Sub)),
        '*' => tokens.push(Token::Op(Operator::Mul)),
        '/' => tokens.push(Token::Op(Operator::Div)),
        ' ' => {}
        '\n' => {}
        _ => return Err(Error::BadToken(c)),
      }
    }

    if parens.len() > 0 {
      // All the parenthesis open are not closed,
      // so we return an error in this case.
      return Err(Error::MismatchedParens);
    }

    Ok(tokens) // We return the tokens matched, if nothing wrong!
  }

  pub fn expression(mut tokens: Vec<Token>) -> Vec<Token> {
    // Given there is no function to retrieve the items at the top,
    // we reverse sequence in first, and then call pop() function
    // on the sequence object.
    tokens.reverse();
    let mut queue: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();
    while let Some(token) = tokens.pop() {
      match token {
        Token::Number(_) => queue.push(token),
        Token::Op(_) => {
          while !stack.is_empty() && stack[stack.len() - 1] >= token {
            // FIXME: It's deprecated to use unwrap() function.
            queue.push(stack.pop().unwrap());
          }
          stack.push(token);
        }
        Token::Bracket('(') => stack.push(token),
        Token::Bracket(')') => {
          while !stack.is_empty()
            && stack[stack.len() - 1] != Token::Bracket('(') {
              // FIXME: It's deprecated to use unwrap() function.
              queue.push(stack.pop().unwrap());
          }
          stack.pop();
        }
        _ => {}
      }
    }

    while stack.len() > 0 {
      queue.push(stack.pop().unwrap());
    }
    queue
  }

  pub fn evaluate(mut tokens: Vec<Token>) -> Option<f32> {
    tokens.reverse();
    let mut stack: Vec<f32> = Vec::new();
    while let Some(token) = tokens.pop() {
      match token {
        Token::Number(num) => stack.push(num as f32),
        Token::Op(Operator::Add) => {
          // FIXME: It's deprecated to use unwrap() function.
          let right = stack.pop().unwrap();
          let left = stack.pop().unwrap();
          stack.push(left + right);
        },
        Token::Op(Operator::Sub) => {
          // FIXME: It's deprecated to use unwrap() function.
          let right = stack.pop().unwrap();
          let left = stack.pop().unwrap();
          stack.push(left - right);
        },
        Token::Op(Operator::Mul) => {
          // FIXME: It's deprecated to use unwrap() function.
          let right = stack.pop().unwrap();
          let left = stack.pop().unwrap();
          stack.push(left * right);
        },
        Token::Op(Operator::Div) => {
          // FIXME: It's deprecated to use unwrap() function.
          let right = stack.pop().unwrap();
          let left = stack.pop().unwrap();
          stack.push(left / right);
        },
        _ => {}  // Nothing else
      }
    }

    if stack.len() == 1 {
      stack.pop()
    } else {
      None
    }
  }
}
