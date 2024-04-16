//--------------------------------------------------------------------------------------------------
// Types
//--------------------------------------------------------------------------------------------------

/// TODO: Document
pub type Result<T> = std::result::Result<T, CryptdagError>;

/// TODO: Document
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CryptdagError {}

//--------------------------------------------------------------------------------------------------
// Functions
//--------------------------------------------------------------------------------------------------

/// TODO: Document
#[allow(non_snake_case)]
pub fn Ok<T>(value: T) -> Result<T> {
    std::result::Result::Ok(value)
}
