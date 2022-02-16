/// Convert trait for turning element into string 
pub trait Convert {
    /// Takes reference of the element and uses it's &self.0 for the string 
    fn to_latex_string(&self) -> String;
}

