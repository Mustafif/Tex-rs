//! Tex-rs <br>
//! Started Development: February 2022 <br>
//! License: MIT <br>
//! From: Mustafif Khan | MKProjects <br>
//! This crate is to be a library to generate LaTeX documents using a concept 
//! of attaching document elements. The example below shows how to utilize this library: 


//! ```
//! use tex_rs::*;
//! use std::path::Path;
//! 
//! fn main() {
//!     let mut latex = latex::Latex::new();
//!     latex.set_class(Class::Book);
//!     latex.set_metadata(Metadata::default());
//!     latex.add_package("dramatist".to_owned());
//! 
//!     let mut part_one = Part::new("Part 1");
//!     let section_one = Section::new("Section 1");
//! 
//!     let mut part_two = Part::new("Part 2");
//!     let mut chapter = Chapter::new("Chapter");
//!     let text = Text::new("text in part 2", TextType::Roman);
//! 
//!     part_one.attach(Element::from(section_one)).unwrap();
//!     chapter.attach(Element::from(text)).unwrap();
//!     part_two.attach(Element::from(chapter)).unwrap();
//! 
//!     let mut env = Environment::new("equation");
//!     env.attach_string("x^2 + y^2 = z^2".to_owned());
//! 
//!     part_two.attach(Element::from(env)).unwrap();
//! 
//!     let mut list = List::new(ListMode::Enumerate, &vec!["item 1".to_owned(), "item 2".to_owned(), "item 3".to_owned()]);
//! 
//!     part_two.attach(Element::from(list)).unwrap();
//! 
//!     latex.set_elements(&vec![Element::from(part_one), Element::from(part_two)]);
//! 
//!     latex.write(Path::new("simple.tex").to_path_buf()).unwrap()
//! }

//! ```

pub use element::*;
pub use error::*;
pub use latex::*;
pub use traits::*;


/// Contains all Element related structs/enums
pub mod element;
/// Contains all custom error handling 
pub mod error;
/// Contains all of the core functionality with the Latex struct
pub mod latex;
/// Contains all of the traits for the structs/enums
pub mod traits;
