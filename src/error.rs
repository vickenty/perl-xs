//! Misc errors

use std::fmt;

/// Error instantiating a rust struct from a perl stack
#[derive(Debug)]
pub struct ToStructErr {
    /// Name of the struct which was not able to be instantiated
    pub name: &'static str,
    /// The errors which were encountered
    pub errors: Vec<ToStructErrPart>,
}

/// Partial error instantiating a rust struct from a perl stack
#[derive(Debug)]
pub enum ToStructErrPart {
    /// A list of keys one of which must be specified
    OmittedKey(&'static [&'static str]),
    /// A key for which a value was not specified
    OmittedValue(&'static str),
    /// Was unable to parse the key of a key-value-pair
    KeyParseFail {
        /// stack offset of the key that was not able to be parsed
        offset: isize,
        /// the type of the field to which the key refers
        ty: &'static str,
        /// Error message returned by the FromSV trait
        error: String,
    },
    /// Information about the failure to parse a value
    ValueParseFail {
        /// The key that was unable to be parsed
        key: &'static str,
        /// stack offset of the value that was not able to be parsed
        offset: isize,
        /// the type of the field to which the key refers
        ty: &'static str,
        /// Error message returned by the FromSV trait
        error: String,
    },
}

impl fmt::Display for ToStructErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ToStructErrPart::*;

        writeln!(f, "Failed to instantiate {}", self.name)?;

        for e in self.errors.iter() {
            match *e {
                OmittedKey(ref names) => {
                    if names.len() == 1 {
                        writeln!(f, "\tMissing field: {}", names[0])?;
                    } else {
                        writeln!(f, "\tMissing one of the following fields: {}", names.join(", "))?;
                    };
                }
                OmittedValue(ref name) => {
                    writeln!(f, "\tValue is required for: {}", name)?;
                }
                KeyParseFail { offset, ref ty, ref error } => {
                    writeln!(f, "\tFailed to parse key at offset {} as {}: {}", offset, ty, error)?;
                }
                ValueParseFail {
                    ref key,
                    ref ty,
                    ref error,
                    ..
                } => {
                    writeln!(f, "\tFailed to parse value of {} as {}: {}", key, ty, error)?;
                }
            }
        }
        write!(f, "")
    }
}
