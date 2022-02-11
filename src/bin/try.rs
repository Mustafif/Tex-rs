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