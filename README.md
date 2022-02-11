# Tex-rs

---
This library is to be a way to create Latex documents with Rust. Currently this is still
a very early build so there's much to add, but here's an example of using the library. 


```rust
use std::path::Path;
use tex_rs::*;
fn main() {
    let mut latex = Latex::new();
    latex.set_class(Class::Article);
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

## Features to be Added
- [ ] Equations
- [ ] Environments
- [ ] Lists
- [ ] Async Version
- [ ] Split Write (Main and Structure)

My goal of this library is to eventually become the backend for my project 
`texcreate`, and to do so I need all of these features implemented. 