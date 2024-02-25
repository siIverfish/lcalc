use core::iter::Peekable;
use std::{panic::PanicInfo, str::Chars};


#[derive(Debug)]
pub enum ParenGroup {
   Group(Vec<ParenGroup>),
   Char(char),
}




pub fn parse_parens(iter: &mut Peekable<Chars>) -> Result<ParenGroup, ()> {
   match iter.next() {
       Some('(') => {
           // listcomp this
           let mut tokens: Vec<ParenGroup> = Vec::new();
           while let Ok(token) = parse_parens(iter) {
               tokens.push(token);
           }


           // there must have just been an error returned
           // the only error case is a closeparen
           // so the group is closed
           Ok(ParenGroup::Group(tokens))
       },
       Some(')') => Err(()),
       Some(x ) => Ok(ParenGroup::Char(x)),
       None => Err(()), // ig running out of space = closeparens
   }
}
