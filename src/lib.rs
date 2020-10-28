
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
    use super::*;

    #[test]
    fn should_handle_nil() {
        let res = parse_data( "nil" );
        assert!( matches!( res, Ok(Data::Nil)) )
    }

    #[test]
    fn should_handle_number() {
        let res = parse_data( "1.2" );

        match res {
            Ok(Data::Number(n)) => assert_eq!( n, "1.2" ),
            _ => panic!( "Expected Number(1.2)" ),
        }
    }

    #[test]
    fn should_handle_string() {
        let res = parse_data( r#""blah""# );

        match res {
            Ok(Data::Str(s)) => assert_eq!( s, "blah" ),
            _ => panic!( "Expected Str(\"blah\")" ),
        }
    }

    #[test]
    fn should_handle_symbol() {
        let res = parse_data( "blah" );

        match res {
            Ok(Data::Symbol(s)) => assert_eq!( s, "blah" ),
            _ => panic!( "Expected Symbol(blah)" ),
        }
    }

    #[test]
    fn should_handle_cons() {
        let res = parse_data( "blah()" );

        match res {
            Ok(Data::Cons{name, params}) => {
                assert_eq!( name, "blah" );
                assert_eq!( params.len(), 0 );
            },
            _ => panic!( "Expected Symbol(blah)" ),
        }
    }

    #[test]
    fn should_handle_cons_with_params() {
        let res = parse_data( "blah(alpha, beta)" );

        match res {
            Ok(Data::Cons{name, params}) => {
                assert_eq!( name, "blah" );
                assert_eq!( params.len(), 2 );
                match &params[0] {
                    Data::Symbol(x) => assert_eq!( x, "alpha" ),
                    _ => panic!( "expected alpha" ),
                }

                match &params[1] {
                    Data::Symbol(x) => assert_eq!( x, "beta" ),
                    _ => panic!( "expected beta" ),
                }
            },
            _ => panic!( "Expected Symbol(blah)" ),
        }
    }

    #[test]
    fn should_handle_cons_with_nested_params() {
        let res = parse_data( "blah(alpha(nest), beta)" );

        match res {
            Ok(Data::Cons{name, params}) => {
                assert_eq!( name, "blah" );
                assert_eq!( params.len(), 2 );
                match &params[0] {
                    Data::Cons{ name, params } => {
                        assert_eq!( name, "alpha" );
                        assert_eq!( params.len(), 1 );
                    },
                    _ => panic!( "expected alpha" ),
                }

                match &params[1] {
                    Data::Symbol(x) => assert_eq!( x, "beta" ),
                    _ => panic!( "expected beta" ),
                }
            },
            _ => panic!( "Expected Symbol(blah)" ),
        }
    }
}
