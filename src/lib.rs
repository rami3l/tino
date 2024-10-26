use thiserror::Error;

#[derive(Debug, Error)]
enum TioError {
    #[error("language {0} not found")]
    LanguageNotFound(String),

    #[error("TIO API error {status}: {reason}")]
    ApiError {
        resp: reqwest::Response,
        status: i32,
        reason: String,
    },
}

/// Model representing the response returned from TIO.
#[derive(Clone, Debug)]
struct TioResponse {
    /// The token of the execution session.
    token: String,
    /// The formatted full output with stdout/stderr and the execution stats.
    output: String,
    /// The programming language that was used for the execution.
    provided_language: String,
    /// The pure stdout of the execution (without execution stats).
    stdout: String,
    /// The total time of execution.
    real_time: f32,
    /// The user time of execution.
    user_time: f32,
    /// The system time of execution.
    sys_time: f32,
    /// The CPU usage taken during execution (as a percentage).
    cpu_usage: f32,
    /// The exit status for the program.
    exit_status: i32,
}

impl PartialEq for TioResponse {
    fn eq(&self, other: &Self) -> bool {
        self.stdout == other.stdout
    }
}

/// Model representing a language available in TIO.
#[derive(Clone, Debug)]
struct Language {
    /// The name of the language TIO expects us to provide for execution.
    tio_name: String,
    /// The actual, raw name of the language.
    name: String,
    /// Some tags for the programming language.
    categories: Vec<String>,
    /// The encoding format of the language, e.g. utf-8.
    encoding: String,
    /// The link to the home page of the language.
    link: String,
    /// A shortened alias for the name of the language.
    alias: String,
}

#[derive(Clone, Debug)]
struct LanguageData {
    name: String,
    categories: Vec<String>,
    encoding: String,
    link: String,
    prettyify: String,
}

impl Language {
    fn new(name: &str, data: LanguageData) -> Self {
        Self {
            tio_name: name.to_owned(),
            name: data.name,
            categories: data.categories,
            encoding: data.encoding,
            link: data.link,
            alias: data.prettyify,
        }
    }
}

impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.tio_name == other.tio_name
    }
}
