use std::io;


// λ
// λa.a
// (λab.a)(λ.ab.a)(λab.b)


use lcalc::parens::parse_parens;
use lcalc::parse;


fn main() -> Result<(), Box<dyn std::error::Error>> {
   let mut input = String::new();
   io::stdin().read_line(&mut input)?;
   // let input = String::from("λa.a");


   let parsed = parse_parens(&mut input.chars().peekable());


   println!("{:#?}", parsed);


   Ok(())
}
