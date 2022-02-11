pub use attatch::*;
pub use error::*;
use std::io::Write;
use std::path::{Path, PathBuf};

pub mod attatch;
pub mod error;

#[derive(Debug, Clone)]
pub struct Latex {
    pub document_class: Class,
    pub metadata: Metadata,
    pub packages: Vec<Package>,
    pub document_elements: Vec<Element>,
}
#[derive(Debug, Clone)]
pub enum Class {
    Article,
    Book,
    Report,
    Beamer,
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub author: String,
    pub title: String,
    pub date: String,
}
#[derive(Debug, Clone)]
pub struct Package(pub String);
#[derive(Debug, Clone)]
pub struct Part(pub String, Vec<Element>);
#[derive(Debug, Clone)]
pub struct Chapter(pub String, Vec<Element>);
#[derive(Debug, Clone)]
pub struct Section(pub String, Vec<Element>);
#[derive(Debug, Clone)]
pub struct Paragraph(pub String, Option<Vec<Element>>);
#[derive(Debug, Clone)]
pub struct Text(pub String, pub TextType);
#[derive(Debug, Clone)]
pub struct Input(pub String);

#[derive(Debug, Clone)]
pub enum Element {
    Part(Part),
    Chapter(Chapter),
    Section(Section),
    Paragraph(Paragraph),
    Text(Text),
    Input(Input),
}
#[derive(Debug, Clone)]
pub enum TextType {
    Normal,
    Bold,
    Italic,
    Underline,
    Verbatim,
    Roman,
}
pub trait Convert {
    fn to_latex_string(&self) -> String;
}

// Implementations //
impl Element {
    pub fn rank(&self) -> u8 {
        match self {
            Element::Part(_) => 0,
            Element::Chapter(_) => 1,
            Element::Section(_) => 2,
            Element::Paragraph(_) => 3,
            Element::Text(_) => 4,
            Element::Input(_) => 5,
        }
    }
}

impl Class {
    pub fn as_class_str(&self) -> String {
        match self {
            Class::Article => "article".to_string(),
            Class::Book => "book".to_string(),
            Class::Report => "report".to_string(),
            Class::Beamer => "beamer".to_string(),
        }
    }
}

impl Metadata {
    pub fn new(title: &str, author: &str, date: &str) -> Self {
        Self {
            title: title.to_string(),
            author: author.to_string(),
            date: date.to_string(),
        }
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Self {
            author: "default author".to_string(),
            title: "default title".to_string(),
            date: "what day is it?".to_string(),
        }
    }
}

impl Latex {
    pub fn new() -> Self {
        Self {
            document_class: Class::Article,
            metadata: Metadata::default(),
            packages: Vec::new(),
            document_elements: Vec::new(),
        }
    }
    pub fn set_class(&mut self, class: Class) {
        self.document_class = class;
    }
    pub fn set_metadata(&mut self, meta: Metadata) {
        self.metadata = meta;
    }
    pub fn set_packages(&mut self, packages: &Vec<Package>) {
        self.packages = packages.to_owned();
    }
    pub fn add_package(&mut self, package: String) {
        self.packages.push(Package(package));
    }
    pub fn set_elements(&mut self, elements: &Vec<Element>) {
        self.document_elements = elements.to_owned();
    }
    pub fn write(&self, path: PathBuf) -> Result<(), std::io::Error> {
        let path = path.as_path();
        let mut file = std::fs::File::create(path)?;
        let mut s: Vec<String> = Vec::new();
        s.push(self.document_class.to_latex_string());
        s.push(self.metadata.to_latex_string());
        for i in &self.packages {
            s.push(i.to_latex_string())
        }
        s.push(String::from("\\begin{document}"));
        s.push(String::from("\\maketitle\n\\newpage"));
        // By attach things should be order by priority
        for i in &self.document_elements {
            match i {
                Element::Part(e) => {
                    s.push(e.to_latex_string());
                    for j in &e.1 {
                        s.push(j.to_latex_string())
                    }
                }
                Element::Chapter(e) => {
                    s.push(e.to_latex_string());
                    for j in &e.1 {
                        s.push(j.to_latex_string())
                    }
                }
                Element::Section(e) => {
                    s.push(e.to_latex_string());
                    for j in &e.1 {
                        s.push(j.to_latex_string())
                    }
                }
                Element::Paragraph(e) => {
                    s.push(e.to_latex_string());
                    for j in e.1.as_ref().unwrap() {
                        s.push(j.to_latex_string())
                    }
                }
                Element::Text(e) => s.push(e.to_latex_string()),
                Element::Input(e) => s.push(e.to_latex_string()),
            }
        }
        s.push(String::from("\\end{document}"));
        let content = s.join("\n");
        file.write_all(&content.as_bytes())?;
        Ok(())
    }
}

impl Part {
    pub fn new(text: &str) -> Self {
        Self(text.to_string(), Vec::new())
    }
}
impl Chapter {
    pub fn new(text: &str) -> Self {
        Self(text.to_string(), Vec::new())
    }
}
impl Section {
    pub fn new(text: &str) -> Self {
        Self(text.to_string(), Vec::new())
    }
}
impl Paragraph {
    pub fn new(text: &str) -> Self {
        Self(text.to_string(), Some(Vec::new()))
    }
}
impl Text {
    pub fn new(text: &str, textType: TextType) -> Self {
        Self(text.to_string(), textType)
    }
}
impl Input {
    pub fn new(text: &str) -> Self {
        Self(text.to_string())
    }
}

impl Package{
    pub fn new(pkg: &str) -> Self{
        Package(pkg.to_string())
    }
}
// Trait Implementations

impl Convert for Class {
    fn to_latex_string(&self) -> String {
        let s = r#"\documentclass[11pt, letterpaper]{_class}"#;
        s.replace("_class", &self.as_class_str())
    }
}

impl Convert for Metadata {
    fn to_latex_string(&self) -> String {
        let v = vec![
            format!("\\author{{{}}}", &self.author),
            format!("\\title{{{}}}", &self.title),
            format!("\\date{{{}}}", &self.date),
        ];
        v.join("\n")
    }
}

impl Convert for Package {
    fn to_latex_string(&self) -> String {
        format!("\\usepackage{{{}}}", &self.0)
    }
}

impl Convert for Part {
    fn to_latex_string(&self) -> String {
        format!("\\part{{{}}}", &self.0)
    }
}

impl Convert for Chapter {
    fn to_latex_string(&self) -> String {
        format!("\\chapter{{{}}}", &self.0)
    }
}

impl Convert for Section {
    fn to_latex_string(&self) -> String {
        format!("\\section{{{}}}", &self.0)
    }
}

impl Convert for Paragraph {
    fn to_latex_string(&self) -> String {
        format!("\\paragraph{{{}}}", &self.0)
    }
}

impl Convert for Text {
    fn to_latex_string(&self) -> String {
        let par = "\\par";
        let mut text = String::new();
        match &self.1 {
            TextType::Normal => text = self.0.clone(),
            TextType::Bold => {
                text = format!("\\textbf{{{}}}", &self.0);
            }
            TextType::Italic => {
                text = format!("\\textit{{{}}}", &self.0);
            }
            TextType::Roman => {
                text = format!("\\textrm{{{}}}", &self.0);
            }
            TextType::Underline => {
                text = format!("\\underline{{{}}}", &self.0);
            }
            TextType::Verbatim => {
                text = format!("\\verb!{}!", &self.0);
            }
        }
        format!("{} {}", par, &text)
    }
}

impl Convert for Input {
    fn to_latex_string(&self) -> String {
        format!("\\input{{{}}}", &self.0)
    }
}

impl Convert for Element {
    fn to_latex_string(&self) -> String {
        match self {
            Element::Part(e) => e.to_latex_string(),
            Element::Chapter(e) => e.to_latex_string(),
            Element::Section(e) => e.to_latex_string(),
            Element::Paragraph(e) => e.to_latex_string(),
            Element::Text(e) => e.to_latex_string(),
            Element::Input(e) => e.to_latex_string(),
        }
    }
}

impl From<Part> for Element {
    fn from(p: Part) -> Self {
        Element::Part(p)
    }
}

impl From<Chapter> for Element{
    fn from(c: Chapter) -> Self {
        Element::Chapter(c)
    }
}

impl From<Section> for Element{
    fn from(s: Section) -> Self {
        Element::Section(s)
    }
}

impl From<Paragraph> for Element{
    fn from(p: Paragraph) -> Self {
        Element::Paragraph(p)
    }
}

impl From<Text> for Element{
    fn from(t: Text) -> Self {
        Element::Text(t)
    }
}

impl From<Input> for Element{
    fn from(i: Input) -> Self {
        Element::Input(i)
    }
}