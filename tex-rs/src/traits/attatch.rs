use crate::Element;
use crate::*;
use crate::error::TexError;

/// Used to attach elements to eachother 
pub trait Attach{
    /// Takes an elemennt and pushes to &self.1
    /// Returns Result with either `()` or rank error 
    fn attach(&mut self, element: Element) -> Result<(), TexError>;
}

impl Attach for Part{
    fn attach(&mut self, element: Element) -> Result<(), TexError>{
        if element.rank() > 0{
           return Ok( self.1.push(element));
        }
        return Err(TexError::RankError);
    }
}

impl Attach for Chapter{
    fn attach(&mut self, element: Element) -> Result<(), TexError>{
        if element.rank() > 1{
            return Ok(self.1.push(element));
        }
        return Err(TexError::RankError);
    }
}

impl Attach for Section{
    fn attach(&mut self, element: Element) -> Result<(), TexError>{
        if element.rank() > 2{
            return Ok( self.1.push(element));
        }
        return Err(TexError::RankError);
    }
}

impl Attach for Paragraph{
    fn attach(&mut self, element: Element) -> Result<(), TexError>{
        if element.rank() > 3{
            return Ok(self.1.as_mut().unwrap().push(element));
        }
        return Err(TexError::RankError);
    }
}

impl Attach for Environment{
    fn attach(&mut self, element: Element) -> Result<(), TexError>{
        if element.rank() > 7{
            return Ok(self.1.push(String::from(element)));
        }
        return Err(TexError::RankError);
    }
}

