use crate::Convert;

// Type aliases //
type Body = String;
type Meta = String;
type Pkg = String;
/// User-Defined Tuple for Body, Meta and Pkg levels 
pub type UDTuple = (Body, Meta, Pkg);
/// Item for Environment and List
pub type Item = String;
// Structs //
/// Part struct, contains text and a vec for elements 
#[derive(Debug, Clone)]
pub struct Part(pub String, pub Vec<Element>);
/// Chapter struct, contains text and a vec for elements 
#[derive(Debug, Clone)]
pub struct Chapter(pub String, pub Vec<Element>);
/// Section struct, contains text and a vec for elements 
#[derive(Debug, Clone)]
pub struct Section(pub String, pub Vec<Element>);
/// Paragraph struct, contains text and a vec for elements 
#[derive(Debug, Clone)]
pub struct Paragraph(pub String, pub Option<Vec<Element>>);
/// Text struct with text and specifid text type 
#[derive(Debug, Clone)]
pub struct Text(pub String, pub TextType);
/// Input struct with text for filename 
#[derive(Debug, Clone)]
pub struct Input(pub String);
/// Environment struct with text for name, and vec for items (String)
#[derive(Debug, Clone)]
pub struct Environment(pub String, pub Vec<Item>);
/// User-defined command with string and level for it to be in 
/// - Meta: In the metadata level, just before packages 
/// - Package: In the level where packages are 
/// - Body: Inside the document environment
#[derive(Debug, Clone)]
pub struct UserDefined(pub String, pub Level);
/// List struct with ListMode (enumerate or itemize) and vec for items (String)
#[derive(Debug, Clone)]
pub struct List(pub ListMode, pub Vec<Item>);

// Enums //

/// Elements enum that contains all elements 
/// - Part 
/// - Chapter
/// - Section (Headers)
/// - Paragraph
/// - Text (Italic, bold, verbatim, etc.)
/// - Input 
/// - Environment `(\begin{}...\end{})`
/// - UserDefiend (any kind of custom code)
/// - List (enumerate or itemize)
#[derive(Debug, Clone)]
pub enum Element {
    Part(Part),
    Chapter(Chapter),
    Section(Section),
    Paragraph(Paragraph),
    Text(Text),
    Input(Input),
    Environment(Environment),
    UserDefined(UserDefined),
    List(List),
}

/// TextType enum that contains the different kind
/// of text like; 
/// - Normal `(\par ...)`
/// - Bold `(\textbf{...})`
/// - Italic `(\textit{...})`
/// - Underline `(\underline{...})`
/// - Verbatim `(\verb!...!)`
/// - Roman `(\textrm{...})`
#[derive(Debug, Clone)]
pub enum TextType {
    Normal,
    Bold,
    Italic,
    Underline,
    Verbatim,
    Roman,
}
/// ListMode enum that contains the two different lists 
/// - Itemize `(\begin{itemize}...\end{itemize})`
/// - Enumerate `(\begin{enumerate}...\end{enumerate})`
#[derive(Debug, Clone)]
pub enum ListMode {
    Itemize,
    Enumerate,
}
/// Level determines where the userdefined command goes 
/// - Meta: Where metadata goes 
/// - Package: Where packages goes 
/// - Body: Inside the document environment 
#[derive(Debug, Clone)]
pub enum Level {
    Meta,
    Body,
    Package,
}

// Implementations //
impl Element {
    /// Determines Prioirty rank 
    pub fn rank(&self) -> u8 {
        match self {
            Element::Part(_) => 0,
            Element::Chapter(_) => 1,
            Element::Section(_) => 2,
            Element::Paragraph(_) => 3,
            Element::UserDefined(_) => 4,
            Element::Text(_) => 8,
            Element::Input(_) => 7,
            Element::Environment(_) => 6,
            Element::List(_) => 5,
        }
    }
    /// Gets the Vec element inside the struct
    pub fn get_vec(&self) -> Option<Vec<Element>> {
        match &self {
            Element::Part(p) => Some(p.to_owned().1),
            Element::Chapter(c) => Some(c.to_owned().1),
            Element::Section(s) => Some(s.to_owned().1),
            Element::Paragraph(p) => Some(p.to_owned().1.unwrap()),
            Element::Text(_) => None,
            Element::Input(_) => None,
            Element::Environment(e) => None,
            Element::UserDefined(_) => None,
            Element::List(_) => None,
        }
    }
    /// Recursive function to get latex string for all elements inside struct 
    pub fn loop_through(&self) -> String {
        let vec = match self.get_vec() {
            Some(a) => a,
            None => return "".to_string(),
        };
        let mut s = Vec::new();
        if vec.is_empty() {
            return "".to_string();
        } else {
            for i in &vec {
                s.push(i.to_latex_string());
                s.push(i.loop_through())
            }
        }
        s.join("\n")
    }
    /// Parallel version of loop_through using rayon
    pub fn loop_through_parallel(&self) -> String {
        let vec = match self.get_vec() {
            Some(a) => a,
            None => return "".to_string(),
        };
        let mut s = Vec::new();
        if vec.is_empty() {
            return "".to_string();
        } else {
            for i in &vec {
                let r = rayon::join(|| i.clone().to_latex_string(), || i.clone().loop_through());
                s.push(r.0);
                s.push(r.1);
            }
        }
        s.join("\n")
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
    pub fn new(text: &str, text_type: TextType) -> Self {
        Self(text.to_string(), text_type)
    }
}
impl Input {
    pub fn new(text: &str) -> Self {
        Self(text.to_string())
    }
}

impl Environment {
    pub fn new(text: &str) -> Self {
        Self(text.to_string(), Vec::new())
    }
    /// Attach string to environment, alternative to attach which requires an Element
    pub fn attach_string(&mut self, item: Item) {
        self.1.push(item);
    }
}

impl List {
    pub fn new(list_mode: ListMode, items: &Vec<Item>) -> Self {
        Self(list_mode, items.to_owned())
    }
}

impl UserDefined {
    pub fn new(text: &str, level: Level) -> Self {
        Self(text.to_string(), level)
    }
    /// Evaluates a userdefined, and puts it's string in the appropriate level 
    pub fn evaluate(&self) -> UDTuple{
        match &self.1{
            Level::Body => {
                (self.0.clone(), "".to_owned(), "".to_owned())
            }
            Level::Meta => {
                ("".to_owned(), self.0.clone(), "".to_owned())
            }, 
            Level::Package => {
                ("".to_owned(), "".to_owned(), self.0.clone())
            }
        }
    }
}

// Trait Implementations //
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

impl Convert for Environment {
    fn to_latex_string(&self) -> String {
        let begin = format!("\\begin{{{}}}", &self.0);
        let end = format!("\\end{{{}}}", &self.0);
        let mut s = Vec::new();
        s.push(begin);
        for i in &self.1 {
            s.push(i.to_owned());
        }
        s.push(end);
        s.join("\n")
    }
}

impl Convert for UserDefined {
    fn to_latex_string(&self) -> String {
        self.0.clone()
    }
}

impl Convert for List {
    fn to_latex_string(&self) -> String {
        let mode: &str = match &self.0 {
            ListMode::Enumerate => "enumerate",
            ListMode::Itemize => "itemize",
        };

        let (begin, end) = (format!("\\begin{{{}}}", mode), format!("\\end{{{}}}", mode));
        let mut s = Vec::new();
        s.push(begin);
        for i in &self.1 {
            s.push(format!("\\item {}", &i));
        }
        s.push(end);
        s.join("\n")
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
            Element::Environment(e) => e.to_latex_string(),
            Element::UserDefined(e) => e.to_latex_string(),
            Element::List(e) => e.to_latex_string(),
        }
    }
}

impl From<Part> for Element {
    fn from(p: Part) -> Self {
        Element::Part(p)
    }
}

impl From<Chapter> for Element {
    fn from(c: Chapter) -> Self {
        Element::Chapter(c)
    }
}

impl From<Section> for Element {
    fn from(s: Section) -> Self {
        Element::Section(s)
    }
}

impl From<Paragraph> for Element {
    fn from(p: Paragraph) -> Self {
        Element::Paragraph(p)
    }
}

impl From<Text> for Element {
    fn from(t: Text) -> Self {
        Element::Text(t)
    }
}

impl From<Input> for Element {
    fn from(i: Input) -> Self {
        Element::Input(i)
    }
}

impl From<Environment> for Element {
    fn from(e: Environment) -> Self {
        Element::Environment(e)
    }
}

impl From<UserDefined> for Element {
    fn from(u: UserDefined) -> Self {
        Element::UserDefined(u)
    }
}

impl From<List> for Element {
    fn from(l: List) -> Self {
        Element::List(l)
    }
}


impl From<Element> for String {
    fn from(e: Element) -> Self {
        e.to_latex_string()
    }
}