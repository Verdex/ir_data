
extern crate parse_input;
extern crate error_reporter;

use parse_input::{Input, ParseError, PSym};
use error_reporter::ErrorReport;

#[derive(Debug)]
pub enum Data {
    Nil, 
    Number(String),
    Str(String),
    Symbol(String),
    Cons { name : String
         , params : Vec<Data>
         },
}

fn parse_nil( parser : &mut Input ) -> Result<Data, ParseError> {
    let v = parser.parse_symbol()?;
    if v.value == "nil" {
        Ok(Data::Nil)
    } 
    else {
        Err(ParseError::ErrorAt(v.start, format!("Expected nil but found {}", v.value)))
    }
}

fn parse( parser : &mut Input ) -> Result<Data, ParseError> {
    let result = parser.choice( &[ parse_nil
                                 , |p| Ok(Data::Number(p.parse_number()?.value))
                                 , |p| Ok(Data::Str(p.parse_string()?.value))
                                 , |p| Ok(Data::Symbol(p.parse_symbol()?.value))
                                 ] );

    Ok(Data::Nil)
}

pub fn parse_data( input : &str ) -> Result<Data, ErrorReport> {

    let i = input.char_indices().collect::<Vec<_>>();
    let mut parser = Input::new( &i );
    
    let result = parse( &mut parser );

    Ok(Data::Nil)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
