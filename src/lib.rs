
extern crate parse_input;

use parse_input::{Input, ParseError};

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

fn parse_cons( parser : &mut Input ) -> Result<Data, ParseError> {
    let name = parser.parse_symbol()?.value;

    parser.expect("(")?;

    let params = parser.list( parse )?;

    parser.expect(")")?;

    Ok( Data::Cons { name, params } )
}

fn parse( parser : &mut Input ) -> Result<Data, ParseError> {
    parser.choice( &[ parse_nil
                    , parse_cons
                    , |p| Ok(Data::Number(p.parse_number()?.value))
                    , |p| Ok(Data::Str(p.parse_string()?.value))
                    , |p| Ok(Data::Symbol(p.parse_symbol()?.value))
                    ] )
}

pub fn parse_data( input : &str ) -> Result<Data, ParseError> {

    let i = input.char_indices().collect::<Vec<_>>();
    let mut parser = Input::new( &i );
    
    parse( &mut parser )
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
