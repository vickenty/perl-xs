use std::fmt::{self, Display};

#[derive(Debug)]
pub struct ToStructErr {
    name: &'static str,
    errors: Vec<ToStructErrPart>
}

#[derive(Debug)]
pub enum ToStructErrPart {
    OmittedField(&'static [&'static str]),
    OmittedValue(&'static str),
    ParseFail(&'static str, &'static str),
}

impl fmt::Display for ToStructErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ToStructErrPart::*;

        writeln!(f, "Failed to instantiate {}", self.name);
        
        for e in self.errors.iter() {
            match *e {
                OmittedField(ref names) => {
                    let s = if names.len() == 1 {
                        writeln!(f,"Missing field: \t{:?}", names[0])
                    }else{
                        writeln!(f,"\tMissing one of the following fields: {:?}", names)
                    };
                },
                OmittedValue(ref name) => {
                    writeln!(f,"\tValue is required for: {}", name)
                },
                ParseFail(ref name, ref err) => {
                    writeln!(f,"\tFailed to parse field {}: {}", name, err);
                }
            }
        }
        write!(f,"")
    }
}