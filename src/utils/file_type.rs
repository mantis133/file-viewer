
#[derive(Debug,Clone)]
pub enum FileType{
// default types
    Directory,
    Txt,
    EXE,
    JPEG,
    PNG,
// slightly cooler but I hate adobe
    PDF,
// microsoft types
    Docx,
    PPTS,
    XLSX,
// programmer types
    JSON,
    Rust,
    Python,
    TOML,
    JavaScript,
    HTML,
    CSS,
    C,
    CPP,
    Java,


// all encompassing type
    UnDefinded
}
 

impl std::fmt::Display for FileType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            FileType::Directory => {write!(f, "Directory")},
            FileType::Txt => {write!(f, "Text")},
            FileType::EXE => {write!(f, "Executable")},
            FileType::JPEG => {write!(f, "Jpeg")},
            FileType::PNG => {write!(f, "Png")},

            FileType::PDF => {write!(f, "PDF")},

            FileType::Docx => {write!(f, "Microsoft Word Document")},
            FileType::PPTS => {write!(f, "Microsoft Powerpoint Presentation")},
            FileType::XLSX => {write!(f, "Microsoft Excel Worksheet")},

            FileType::JSON => {write!(f, "Json")},
            FileType::Rust => {write!(f, "Rust")},
            FileType::Python => {write!(f, "Python")},
            FileType::TOML => {write!(f, "TOML")},
            FileType::JavaScript => {write!(f, "JavaScript")},
            FileType::HTML => {write!(f, "HTML")},
            FileType::CSS => {write!(f, "Css")},
            FileType::C => {write!(f, "C")},
            FileType::CPP => {write!(f, "C++")},
            FileType::Java => {write!(f, "Java")},

            FileType::UnDefinded => {write!(f, "UnDefined")},
        }
    }
}