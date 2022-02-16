# Tex-rs

---
This library is to be a way to create Latex documents with Rust. This library will be used
as the backend for texcreate, and can be used for any desires of your own as well. 

Version 0.2 introduces 
- Environments
- Lists 
- Userdefined code 
- Fixes to write which now uses `loop_through`
- Async and Split write options 
  - Uses `loop_through_parallel` for async 
  - Uses `loop_through` for split 
- Class options 

## Simple Example 
```rust
use std::path::Path;
use tex_rs::*;
fn main() {
    let mut latex = Latex::new();
    latex.set_class(Class::Book);
    latex.set_class_options(11, "letterpaper");
    latex.set_metadata(Metadata::new("A title", "An author", "What day is it?"));
    latex.add_package("dramatist".to_string());
    latex.add_package("listings".to_string());

    let mut part = Part::new("Part 1");
    let mut chapter = Chapter::new("Chapter 1");
    let mut section = Section::new("Section 1");
    let mut text = Text::new("Some text", TextType::Underline);

    section.attach(Element::from(text)).unwrap();
    chapter.attach(Element::from(section)).unwrap();
    part.attach(Element::from(chapter)).unwrap();

    latex.set_elements(&vec![Element::from(part)]);
    latex.write(Path::new("test.tex").to_path_buf()).unwrap();
}
```

## Async Example 
```rust
use std::path::Path;
use tex_rs::*;

#[tokio::main]
async fn main(){
    let mut latex = Latex::new();
    latex.set_class(Class::Article);
    latex.set_metadata(Metadata::default());
    latex.add_package("listings".to_string());

    let mut equation = Environment::new("equation");
    environment.attach_string("a^2 + b^2 = c^2");

    /* 
    Let say you wanted some really long code and don't want 
    to do all of the attaching 
    */

    let cmd: &str = r#"\begin{equation*}
    \begin{split}
        k &= 90l + u78\\
        % random math
        &= 989696
    \end{split}
    \end{equation*}
    "#;

    let ud = UserDefined::new(cmd, Level::Body);

    let mut section = Section::new("The Pythagorean Theorem");
    section.attach(Element::from(equation));
    latex.attach(Element::from(ud));
    latex.set_elements(&vec![Element::from(section)]);

    latex.async_write(Path::new("async_doc.tex").to_path_buf()).await.unwrap();
    // If you want a split write like in texcreate 
    // latex.split_write(
    //    Path::new("main.tex").to_path_buf(), 
    //    Path::new("structure.tex").to_path_buf(),
    // ).await.unwrap()
}
```
