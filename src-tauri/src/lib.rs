use parser::constants::{Constant, Target};

mod parser;
mod logger;

#[tauri::command]
async fn calculate(expression: &str, target: Target) -> Result<Constant, String> {
  let mut parser = parser::calculate::Parser::new(expression);
  parser.execute(target)
}

#[cfg(test)]
mod tests{
  use crate::parser::constants::{Target, Constant};
  #[test]
  fn test_arithmetic_integers() {
    use crate::parser::calculate::Parser;
    let mut parser = Parser::new("1 + 1 * 2");
    let result = parser.execute(Target::Integer);
    let expected = Constant::Integer(3);
    assert_eq!(result, Ok(expected));
  }
  #[test]
  fn test_arithmetic_floats() {
    use crate::parser::calculate::Parser;
    let mut parser = Parser::new("1.5 + 1 * 2.0");
    let result = parser.execute(Target::Float);
    let expected = Constant::Float(3.5);
    assert_eq!(result, Ok(expected));
  }
  #[test]
  fn test_arithmetic_parenthesis() {
    use crate::parser::calculate::Parser;
    let mut parser = Parser::new("(-1 + 0b1) - 3.0");
    let result = parser.execute(Target::Hexadecimal);
    let expected = Constant::Hexadecimal((0xc008000000000000 as u64) as i64);
    assert_eq!(result, Ok(expected));
  }
  #[test]
  fn test_hex_2_octal() {
    use crate::parser::calculate::Parser;
    let mut parser = Parser::new("0xc00800000000000");
    let result = parser.execute(Target::Octal);
    let expected = Constant::Octal(0o60004000000000000000);
    assert_eq!(result, Ok(expected));
  }
  #[test]
  fn test_float_2_octal_2_hex() {
    use crate::parser::calculate::Parser;
    let mut parser = Parser::new("0.5 + 0.5");
    let mut result = parser.execute(Target::Octal);
    let mut expected = Constant::Octal(0o377600000000000000000);
    assert_eq!(result, Ok(expected));
    parser = Parser::new("0.5 + 0.5");
    result = parser.execute(Target::Hexadecimal);
    expected = Constant::Hexadecimal(0x3ff0000000000000);
    assert_eq!(result, Ok(expected));
    parser = Parser::new("0x3ff0000000000000");
    result = parser.execute(Target::Octal);
    expected = Constant::Octal(0o377600000000000000000);
    assert_eq!(result, Ok(expected));
  }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![calculate])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
