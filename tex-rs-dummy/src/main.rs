use tex_rs::*;
use std::path::Path;
#[tokio::main]
async fn main() {
    let mut latex = latex::Latex::new();
    latex.set_class(Class::Book);
    latex.set_metadata(Metadata::default());
    latex.add_package("dramatist".to_owned());

    let mut part_one = Part::new("Part 1");
    let section_one = Section::new("Section 1");

    let mut part_two = Part::new("Part 2");
    let mut chapter = Chapter::new("Chapter");
    let text = Text::new("text in part 2", TextType::Roman);

    part_one.attach(Element::from(section_one)).unwrap();
    chapter.attach(Element::from(text)).unwrap();
    part_two.attach(Element::from(chapter)).unwrap();

    let mut env = Environment::new("equation");
    env.1.push("x^2 + y^2 = z^2".to_owned());

    part_two.attach(Element::from(env)).unwrap();


    let mut list = List::new(ListMode::Enumerate, &vec!["item 1".to_owned(), "item 2".to_owned(), "item 3".to_owned()]);

    part_two.attach(Element::from(list)).unwrap();

    let ud = UserDefined::new("Some package defined stuff", Level::Package);

    latex.set_elements(&vec![Element::from(part_one), Element::from(part_two), Element::from(ud)]);

    latex.async_write(Path::new("async_tex.tex").to_path_buf()).await.unwrap()
}
