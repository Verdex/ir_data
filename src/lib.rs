
extern crate parse_input;
extern crate error_reporter;

use parse_input::{Input, ParseError, PSym};
use error_reporter::ErrorReport;

#[derive(Debug)]
pub enum Data {
    Nil, 
    Cons { name : String
         , params : Vec<Data>
         },

}

pub fn parse_data( input : &str ) -> Result<Data, ErrorReport> {
    Ok(Data::Nil)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
