//! Misc errors

use std::fmt;

/// Error instantiating a rust struct from a perl stack
#[derive(Debug)]
pub struct ToStructErr {
    name: &'static str,
    errors: Vec<ToStructErrPart>
}

/// Partial error instantiating a rust struct from a perl stack
#[derive(Debug)]
pub enum ToStructErrPart {
    /// A list of keys one of which must be specified
    OmittedKey(&'static [&'static str]),
    /// A key for which a value was not specified
    OmittedValue(&'static str),
    /// Information about the failure to parse a key
    ParseFail{
        /// The key that was unable to be parsed
        key:   &'static str,
        /// the type of the field to which the key refers
        ty:    &'static str,
        /// Error message returned by the FromSV trait
        error: &'static str,
    },
}

impl fmt::Display for ToStructErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ToStructErrPart::*;

        writeln!(f, "Failed to instantiate {}", self.name);
        
        for e in self.errors.iter() {
            match *e {
                OmittedKey(ref names) => {
                    if names.len() == 1 {
                        writeln!(f,"Missing field: \t{:?}", names[0]);
                    }else{
                        writeln!(f,"\tMissing one of the following fields: {:?}", names);
                    };
                },
                OmittedValue(ref name) => {
                    writeln!(f,"\tValue is required for: {}", name);
                },
                ParseFail{ ref key, ref ty, ref error } => {
                    writeln!(f,"\tFailed to parse field {} as {}: {}", key, ty, error);
                }
            }
        }
        write!(f,"")
    }
}