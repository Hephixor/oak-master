#![feature(plugin)]
#![plugin(oak)]

extern crate oak_runtime;
use oak_runtime::*;

grammar! json {
  // #![show_api]

  well_formed_json = lbrace json_value* rbrace
  expression
    = term (term_op term)* > fold_left

  term
    = exponent (factor_op exponent)* > fold_left

  exponent
    = (factor exponent_op)* factor > fold_right

  factor
    = number > number_expr
    / lparen expression rparen


  term_op
    = add_op > add_bin_op
    / sub_op > sub_bin_op

  factor_op
    = mul_op > mul_bin_op
    / div_op > div_bin_op

  exponent_op = exp_op > exp_bin_op

  json_value
    = json_array

  json_array
    = lbracket json_chars (("," json_chars)+)? rbracket
    / lbracket number (("," number)+)? rbracket
    / lbracket json_object (("," json_object)+)? rbracket
    ((lbracket json_chars (("," json_chars)+)? rbracket
    / lbracket number (("," number)+)? rbracket
    / lbracket json_object (("," json_object)+)? rbracket)+)?

  json_object
    = lbrace json_members? rbracket

  json_members
    = json_pair (("," json_pair)+)?

  json_pair
    = json_string ":" json_string
    / json_string ":" number
    / json_string ":" json_object
    / json_string ":" json_array
    / json_string ":" json_string
    / json_string ":" "True"
    / json_string ":" "False"
    / json_string ":" "null"

  spacing = [" \n\r\t"]* -> (^)
  digit = ["0-9"]
  number = digit+ spacing > to_number
  not_zero_digit = ["1-9"]
  digits = digit+
 // not_zero_number = not_zero_digit digit+ spacing > to_number
  negative_number = "-" number
  negative_digit = "-" not_zero_digit

  json_string
    = "\"" json_chars "\""

  json_chars
    = json_char (json_char+)?

  json_char
    = ["a-zA-Z"]

  // json_char
  //   = ["a-zA-Z"]
  //   / "\""
  //   / "\\"
  //   / "\/"
  //   / "\b"
  //   / "\f"
  //   / "\n"
  //   / "\r"
  //   / "\t"
  //   / "\u{digit digit digit digit}"

  json_number
    = int exp?

  int
    = digit
    / negative_digit

  ints
    = not_zero_digit digits
    / "-" not_zero_digit digits

  //frac = "." digits > to_number // erreur converti float en int ?
  exp
    = e digits
  e
    = "e"
    / "e+"
    / "e-"
    / "E"
    / "E+"
    / "E-"




  bind_op = "=" spacing
  add_op = "+" spacing
  sub_op = "-" spacing
  mul_op = "*" spacing
  div_op = "/" spacing
  exp_op = "^" spacing
  lparen = "(" spacing
  rparen = ")" spacing
  lbracket = "[" spacing
  rbracket = "]" spacing
  lbrace = "{" spacing
  rbrace = "}" spacing

  use std::str::FromStr;
  use self::Expression::*;
  use self::BinOp::*;

  pub type PExpr = Box<Expression>;

  #[derive(Debug)]
  pub enum Expression {
    Variable(String),
    Number(u32),
    BinaryExpr(BinOp, PExpr, PExpr),
    LetIn(String, PExpr, PExpr)
  }

  #[derive(Debug)]
  pub enum BinOp {
    Add, Sub, Mul, Div, Exp
  }

  #[derive(Debug)]
  pub enum Bool {
    True, False
  }

  fn to_number(raw_text: Vec<char>) -> u32 {
    u32::from_str(&*to_string(raw_text)).unwrap()
  }

  fn number_expr(value: u32) -> PExpr {
    Box::new(Number(value))
  }


  fn to_string(raw_text: Vec<char>) -> String {
    raw_text.into_iter().collect()
  }

  fn fold_left(head: PExpr, rest: Vec<(BinOp, PExpr)>) -> PExpr {
    rest.into_iter().fold(head,
      |accu, (op, expr)| Box::new(BinaryExpr(op, accu, expr)))
  }

  fn fold_right(front: Vec<(PExpr, BinOp)>, last: PExpr) -> PExpr {
    front.into_iter().rev().fold(last,
      |accu, (expr, op)| Box::new(BinaryExpr(op, expr, accu)))
  }


  fn add_bin_op() -> BinOp { Add }
  fn sub_bin_op() -> BinOp { Sub }
  fn mul_bin_op() -> BinOp { Mul }
  fn div_bin_op() -> BinOp { Div }
  fn exp_bin_op() -> BinOp { Exp }
}


// fn analyse_state(state: ParseState<StrStream, json::PExpr>) {
//   use oak_runtime::parse_state::ParseResult::*;
//   match state.into_result() {
//     Success(data) => println!("Full match: {:?}", data),
//     Partial(data, expectation) => {
//       println!("Partial match: {:?} because: {:?}", data, expectation);
//     }
//     Failure(expectation) => {
//       println!("Failure: {:?}", expectation);
//     }
//   }
// }
//
// #[test]
// fn parse_json(){
//     let json_fail =" 2 * 1 }";
//     //println!("{:?}", json::parse_program(json_fail.into_state()).into_result());
//     analyse_state(json::parse_well_formed_json(json_fail.into_state()));
//
// }

fn main() {
  //analyse_state(json::parse_program("{ 2 + 4 }".into_state())); // Complete
  // analyse_state(json::parse_program("2 *  ".into_state())); // Partial
  // analyse_state(json::parse_program("  * a".into_state())); // Erroneous

  // let json_pass ="{1}";
  // analyse_state(json::parse_well_formed_json(json_pass.into_state()));
  //
  // let json_fail =" 2 * 1 }";
  // //println!("{:?}", json::parse_program(json_fail.into_state()).into_result());
  // analyse_state(json::parse_well_formed_json(json_fail.into_state()));

}
