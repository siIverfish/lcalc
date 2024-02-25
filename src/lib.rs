use std::str::Chars;
use std::iter::Peekable;


pub mod parens;


// λ
// (λab.a)(λ.ab.a)(λab.b)


// EndStatement exception?


#[derive(Debug)]
pub struct LFunction(LName, Box<LToken>);


#[derive(Debug)]
pub struct LApplication(Box<(LToken, LToken)>);
// todo: replace all `panic`s with ParseErrors


impl LFunction {
   fn parse(iter: &mut Peekable<Chars>) -> LFunction {
       // get bound variable
       // should ALWAYS succeed on this parse
       // assuming input ∈ Λ
       let bound_variable: LName = LName::parse(iter).unwrap();


       println!("bound var: {:#?}, iter: {:#?}", bound_variable, iter.clone().collect::<String>());


       // get expression
       if LName::parsable(iter) {
           // supports automatic currying
           // the next character is a name, e.g.
           // λab.x
           //   ^
           LFunction(
               bound_variable,
               Box::new(
                   LToken::Function(LFunction::parse(iter))
               )
           )
       } else {
           // the next character is not a char/name.
           // in this case, it must have been the period
           // λa.x
           //   ^
           // so, the next segment must be an expression
           // this assumes that () parsing has already occurred
           assert_eq!(iter.next(), Some('.'), "consume period of lambda expression");


           LFunction(
               bound_variable,
               Box::new(
                   parse_iter(iter)
               )
           )
       }
   }
}


#[derive(Debug)]
pub struct LTokenGroup(Vec<LToken>);


impl LTokenGroup {
   fn parse(iter: &mut Peekable<Chars>) -> LTokenGroup {
       let Some('(') = iter.next() else { panic!("incorrect LTokenGroup parsing") }; // better in future


       println!("LTokenGroup::parse");


       let mut tokens: Vec<LToken> = Vec::new();


       loop {
           println!("LTokenGroup::parse parsing once");
           let new_token = parse_iter_once(iter);
           tokens.push(new_token);
           if iter.peek() == Some(&')') { break; }
       }


       let Some(')') = iter.next() else { unreachable!() };




       println!("Done parsing LTokenGroup, remainder: {:#?}", iter.clone().collect::<String>());


       LTokenGroup( tokens )
   }
}


// struct


#[derive(Debug, Eq, PartialEq)]
pub struct LName(char);


impl LName {
   fn parsable(iter: &mut Peekable<Chars>) -> bool {
       iter.peek().filter(|c| c.is_alphabetic()).is_some()
   }


   fn parse(iter: &mut Peekable<Chars>) -> Option<LName> {
       // slightly risky because always consumes the next character of the iterable.
       // could change with peekable()?


       iter
           .next()
           .filter(|char| (*char).is_alphabetic())
           .map(LName)
   }
}


#[derive(Debug)]
pub enum LToken {
   Function(LFunction),
   TokenGroup(LTokenGroup),
   Application(LApplication),
   Name(LName),
}


pub fn parse(input: &str) -> LToken {
   let mut iter = input.chars().peekable();


   parse_iter(&mut iter)
}


fn parse_iter(iter: &mut Peekable<Chars>) -> LToken {
   let token = parse_iter_once(iter);


   if iter.peek().is_some() {
       let next_token = parse_iter(iter);
       LToken::Application(
           LApplication(
               Box::new((token, next_token))
           )
       )
   } else {
       token
   }
}


fn parse_iter_once(iter: &mut Peekable<Chars>) -> LToken {
   println!("parsing iter {:#?}", iter.clone().collect::<String>());


   // e.g. "λabc.a + b + c"
   if let Some('λ') = iter.peek() {
       assert_eq!(iter.next(), Some('λ'), "consume λ of lambda expression");
       // LToken::Function
       LToken::Function(LFunction::parse(iter))


   } else if let Some('(') = iter.peek() {
       // LToken::TokenGroup
       LToken::TokenGroup(LTokenGroup::parse(iter))


   } else if iter
           .peek()
           .filter( |char: &&char| (**char).is_alphabetic() ) // is this how i'm supposed to use pointers
           .is_some() {
       // LToken::Name
       LToken::Name(LName::parse(iter).unwrap())


   } else {
       // LToken::Application ?
       println!("{}", iter.collect::<String>());
       unimplemented!();
   }
}
