use crate::element::{Element, UDTuple};
use crate::Convert;
use async_std::fs::File;
use async_std::io::WriteExt;
use std::io::Write;
use std::path::PathBuf;

type Doc = (Class, Option<u8>, Option<String>);

/// Latex struct that contains everything related to a latex document 
#[derive(Debug, Clone)]
pub struct Latex {
    /// Document class contains: 
    /// - Class `(\documentclass{...})`
    /// - Fontsize `(\documentclass[..pt]{...})`
    /// - Papersize `(\documentclass[..pt, ...]{...})`
    pub document_class: Doc,
    /// Metadata contains the author, title and date 
    pub metadata: Metadata,
    /// Packages contains all the packages `(\usepackage{...})`
    pub packages: Vec<Package>,
    /// Document elements contains a vector of all the elements
    pub document_elements: Vec<Element>,
}

/// Different kind of document classes 
#[derive(Debug, Clone)]
pub enum Class {
    /// `\documentclass{article}`
    Article,
    /// `\documentclass{book}`
    Book,
    /// `\documentclass{report}`
    Report,
    /// `\documentclass{beamer}`
    Beamer,
}
/// The metadata in a latex document 
#[derive(Debug, Clone)]
pub struct Metadata {
    /// `\author{...}`
    pub author: String,
    /// `\title{...}`
    pub title: String,
    /// `\date{...}`
    pub date: String,
}
/// Package tuple struct 
#[derive(Debug, Clone)]
pub struct Package(pub String);

// Implementations //
impl Package {
    pub fn new(pkg: &str) -> Self {
        Package(pkg.to_string())
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
            document_class: (Class::Article,Some(11), Some("letterpaper".to_string())),
            metadata: Metadata::default(),
            packages: Vec::new(),
            document_elements: Vec::new(),
        }
    }
    /// Sets the class for Latex Document 
    pub fn set_class(&mut self, class: Class) {
        self.document_class.0 = class;
    }
    /// Sets the class options for Latex Document 
    pub fn set_class_options(&mut self, font_size: u8, paper_size: &str) {
        self.document_class.1 = Some(font_size);
        self.document_class.2 = Some(paper_size.to_string());
    }
    /// Sets the metadata for the Latex Document 
    pub fn set_metadata(&mut self, meta: Metadata) {
        self.metadata = meta;
    }
    /// Sets the packages for the Latex Document
    pub fn set_packages(&mut self, packages: &Vec<Package>) {
        self.packages = packages.to_owned();
    }
    /// Adds a single package to the packages 
    pub fn add_package(&mut self, package: String) {
        self.packages.push(Package(package));
    }
    /// Sets the elements for the Latex Document
    pub fn set_elements(&mut self, elements: &Vec<Element>) {
        self.document_elements = elements.to_owned();
    }
    /// Returns a vector of UDTuple, used for write so they 
    /// can be written in the correct location 
    pub fn get_ud(&self) -> Vec<UDTuple>{
        let mut v: Vec<UDTuple> = Vec::new();
        
        for i in &self.document_elements{
            match i{
                Element::UserDefined(u) => {
                    v.push(u.evaluate())
                }
                _ => {
                    v.push(("".to_owned(), "".to_owned(), "".to_owned()))
                }
            }
        }
        v
    }
    /// Normal write that uses `std`
    pub fn write(&self, path: PathBuf) -> Result<(), std::io::Error> {
        let ud_vec = &self.get_ud();
        let path = path.as_path();
        let mut file = std::fs::File::create(path)?;
        let mut s: Vec<String> = Vec::new();
        s.push(self.document_class.to_latex_string());
        s.push(self.metadata.to_latex_string());
        for i in ud_vec{
            s.push(i.1.to_owned())
        }
        for i in &self.packages {
            s.push(i.to_latex_string())
        }
        for i in ud_vec{
            s.push(i.2.to_owned())
        }
        s.push(String::from("\\begin{document}"));
        s.push(String::from("\\maketitle\n\\newpage"));
        // By attach things should be order by priority
        for i in &self.document_elements {
            match i {
                Element::Part(e) => {
                    s.push(e.to_latex_string());
                    for j in &e.1 {
                        s.push(j.to_latex_string());
                        s.push(j.loop_through())
                    }
                }
                Element::Chapter(e) => {
                    s.push(e.to_latex_string());
                    for j in &e.1 {
                        s.push(j.to_latex_string());
                        s.push(j.loop_through())
                    }
                }
                Element::Section(e) => {
                    s.push(e.to_latex_string());
                    for j in &e.1 {
                        s.push(j.to_latex_string());
                        s.push(j.loop_through())
                    }
                }
                Element::Paragraph(e) => {
                    s.push(e.to_latex_string());
                    for j in e.1.as_ref().unwrap() {
                        s.push(j.to_latex_string());
                        s.push(j.loop_through())
                    }
                }
                Element::Environment(e) => {
                    s.push(e.to_latex_string());
                }
                Element::List(e) => s.push(e.to_latex_string()),
                Element::UserDefined(e) => {
                    let ud = e.evaluate();
                    s.push(ud.0)
                },
                Element::Text(e) => s.push(e.to_latex_string()),
                Element::Input(e) => s.push(e.to_latex_string()),
            }
        }
        s.push(String::from("\\end{document}"));
        let content = s.join("\n");
        file.write_all(&content.as_bytes())?;
        Ok(())
    }
    /// Asynchronous and parallel write using `async_std` and `rayon`
    pub async fn async_write(&self, path: PathBuf) -> Result<(), async_std::io::Error> {
        let ud_vec = &self.get_ud();
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(4)
            .build()
            .unwrap();
        let path = path.as_path();
        let mut file = File::create(path).await?;
        let mut s: Vec<String> = Vec::new();
        s.push(self.document_class.to_latex_string());
        s.push(self.metadata.to_latex_string());
        for i in ud_vec{
            s.push(i.1.to_owned())
        }
        for i in &self.packages {
            s.push(i.to_latex_string())
        }
        for i in ud_vec{
            s.push(i.2.to_owned())
        }
        s.push(String::from("\\begin{document}"));
        s.push(String::from("\\maketitle\n\\newpage"));
        // By attach things should be order by priority
        for i in &self.document_elements {
            match i {
                Element::Part(e) => {
                    s.push(e.to_latex_string());
                    for j in &e.1 {
                        s.push(j.to_latex_string());
                        s.push(pool.install(|| j.clone().loop_through_parallel()))
                    }
                }
                Element::Chapter(e) => {
                    s.push(e.to_latex_string());
                    for j in &e.1 {
                        s.push(j.to_latex_string());
                        s.push(pool.install(|| j.clone().loop_through_parallel()))
                    }
                }
                Element::Section(e) => {
                    s.push(e.to_latex_string());
                    for j in &e.1 {
                        s.push(j.to_latex_string());
                        s.push(pool.install(|| j.clone().loop_through_parallel()))
                    }
                }
                Element::Paragraph(e) => {
                    s.push(e.to_latex_string());
                    for j in e.1.as_ref().unwrap() {
                        s.push(j.to_latex_string());
                        s.push(pool.install(|| j.clone().loop_through_parallel()))
                    }
                }
                Element::Environment(e) => {
                    s.push(e.to_latex_string());
                }
                Element::List(e) => s.push(e.to_latex_string()),
                Element::UserDefined(e) => {
                    let ud = e.evaluate();
                    s.push(ud.0)
                },
                Element::Text(e) => s.push(e.to_latex_string()),
                Element::Input(e) => s.push(e.to_latex_string()),
            }
        }
        s.push(String::from("\\end{document}"));
        let content = s.join("\n");
        file.write_all(&content.as_bytes()).await?;
        Ok(())
    }
    /// Split write that writes a `main` and `structure` file 
    /// Uses `async_std` to do write asynchronously 
    pub async fn split_write(
        &self,
        main: PathBuf,
        structure: PathBuf,
    ) -> Result<(), async_std::io::Error> {
        let ud_vec = &self.get_ud();
        let main = main.as_path();
        let structure = structure.as_path();

        let mut s: Vec<String> = Vec::new();
        let mut struct_s: Vec<String> = Vec::new();

        s.push(self.document_class.to_latex_string());
        s.push(self.metadata.to_latex_string());
        for i in ud_vec{
            s.push(i.1.to_owned())
        }
        for i in &self.packages {
            struct_s.push(i.to_latex_string())
        }
        for i in ud_vec{
            struct_s.push(i.2.to_owned())
        }
        s.push(String::from("\\begin{document}"));
        s.push(String::from("\\maketitle\n\\newpage"));
        // By attach things should be order by priority
        for i in &self.document_elements {
            match i {
                Element::Part(e) => {
                    s.push(e.to_latex_string());
                    for j in &e.1 {
                        s.push(j.to_latex_string());
                        s.push(j.loop_through())
                    }
                }
                Element::Chapter(e) => {
                    s.push(e.to_latex_string());
                    for j in &e.1 {
                        s.push(j.to_latex_string());
                        s.push(j.loop_through())
                    }
                }
                Element::Section(e) => {
                    s.push(e.to_latex_string());
                    for j in &e.1 {
                        s.push(j.to_latex_string());
                        s.push(j.loop_through())
                    }
                }
                Element::Paragraph(e) => {
                    s.push(e.to_latex_string());
                    for j in e.1.as_ref().unwrap() {
                        s.push(j.to_latex_string());
                        s.push(j.loop_through())
                    }
                }
                Element::Environment(e) => {
                    s.push(e.to_latex_string());
                }
                Element::List(e) => s.push(e.to_latex_string()),
                Element::UserDefined(e) => {
                    let ud = e.evaluate();
                    s.push(ud.0)
                },
                Element::Text(e) => s.push(e.to_latex_string()),
                Element::Input(e) => s.push(e.to_latex_string()),
            }
        }
        s.push(String::from("\\end{document}"));
        let content = s.join("\n");
        let mut main_file = File::create(main).await?;
        main_file.write_all(&content.as_bytes()).await?;

        let structure_cont = struct_s.join("\n");
        let mut struct_file = File::create(structure).await?;
        struct_file.write_all(&structure_cont.as_bytes()).await?;
        Ok(())
    }
}

// Trait Implementations
impl Convert for Doc{
    fn to_latex_string(&self) -> String {
        let fs = match self.1.clone(){
            Some(f) => f.to_string(), 
            None => "11".to_owned()
        };
        let paper = match self.2.clone(){
            Some(a) => a,
            None => "letterpaper".to_owned() 
        };
        format!("\\documentclass[{}pt, {}]{{{}}}", &fs, &paper, &self.0.as_class_str())
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
