use std::str::FromStr;

const HELP: &str = "
    To get commit messages in the conventional format, use `--format git-msg`

    The default is a clean table in stdout
";

#[derive(Debug, Clone)]
pub enum Format {
    Table,
    GitCommitMsg,
}

#[derive(Fail, Debug)]
pub enum FormatError {
    #[fail(display = "Provided: {}\n{}", given, valid)]
    Invalid { given: String, valid: String },
}

impl FromStr for Format {
    type Err = FormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "table" => Ok(Format::Table),
            "git-msg" => Ok(Format::GitCommitMsg),
            _ => Err(FormatError::Invalid {
                given: s.to_string(),
                valid: HELP.to_string(),
            }),
        }
    }
}

impl Default for Format {
    fn default() -> Self {
        Format::Table
    }
}

impl std::string::ToString for Format {
    fn to_string(&self) -> String {
        match self {
            Format::Table => "table",
            Format::GitCommitMsg => "git-msg",
        }
        .to_string()
    }
}
