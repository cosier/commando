use std::error::Error;
use std::fmt;
use std::io;
use std::num;
use std::process::{Output, ExitStatus};
use std::str;
use std::string;

use curl;
use git2;
use semver;
use serde_json;
use term;
// use toml;

error_chain! {
    types {
        CommandoError, CommandoErrorKind, CommandoResultExt, CommandoResult;
    }

    foreign_links {
        ParseSemver(semver::ReqParseError);
        Semver(semver::SemVerError);
        Io(io::Error);
        SerdeJson(serde_json::Error);
        Term(term::Error);
        ParseInt(num::ParseIntError);
        ParseBool(str::ParseBoolError);
        Parse(string::ParseError);
        Git(git2::Error);
        Curl(curl::Error);
    }

    errors {
        Internal(err: Box<CommandoErrorKind>) {
            description(err.description())
                display("{}", *err)
        }
        ProcessErrorKind(proc_err: ProcessError) {
            description(&proc_err.desc)
                display("{}", &proc_err.desc)
        }
        HttpNot200(code: u32, url: String) {
            description("failed to get a 200 response")
                display("failed to get 200 response from `{}`, got {}", url, code)
        }
    }
}

impl CommandoError {
    pub fn into_internal(self) -> Self {
        CommandoError(CommandoErrorKind::Internal(Box::new(self.0)), self.1)
    }

    fn is_human(&self) -> bool {

        match &self.0 {
            &CommandoErrorKind::Msg(_) => true,
            &CommandoErrorKind::Curl(_) => true,
            &CommandoErrorKind::HttpNot200(..) => true,
            &CommandoErrorKind::ProcessErrorKind(_) => true,
            &CommandoErrorKind::ParseSemver(_) |
            &CommandoErrorKind::Semver(_) |
            &CommandoErrorKind::Io(_) |
            &CommandoErrorKind::SerdeJson(_) |
            &CommandoErrorKind::Term(_) |
            &CommandoErrorKind::ParseInt(_) |
            &CommandoErrorKind::ParseBool(_) |
            &CommandoErrorKind::Parse(_) |
            &CommandoErrorKind::Git(_) |
            &CommandoErrorKind::Internal(_) => false,
        }
    }
}


// =============================================================================
// Process errors
#[derive(Debug)]
pub struct ProcessError {
    pub desc: String,
    pub exit: Option<ExitStatus>,
    pub output: Option<Output>,
}

// =============================================================================
// CLI errors
pub type CliResult = Result<(), CliError>;

#[derive(Debug)]
pub struct CliError {
    pub error: Option<CommandoError>,
    pub unknown: bool,
    pub exit_code: i32,
}

impl Error for CliError {
    fn description(&self) -> &str {
        self.error.as_ref().map(|e| e.description()).unwrap_or(
            "unknown cli error",
        )
    }

    fn cause(&self) -> Option<&Error> {
        self.error.as_ref().and_then(|e| e.cause())
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref error) = self.error {
            error.fmt(f)
        } else {
            self.description().fmt(f)
        }
    }
}

impl CliError {
    pub fn new(error: CommandoError, code: i32) -> CliError {
        let human = &error.is_human();
        CliError {
            error: Some(error),
            exit_code: code,
            unknown: !human,
        }
    }

    pub fn code(code: i32) -> CliError {
        CliError {
            error: None,
            exit_code: code,
            unknown: false,
        }
    }
}

impl From<CommandoError> for CliError {
    fn from(err: CommandoError) -> CliError {
        CliError::new(err, 101)
    }
}


// =============================================================================
// Construction helpers

pub fn process_error(
    msg: &str,
    status: Option<&ExitStatus>,
    output: Option<&Output>,
) -> ProcessError {
    let exit = match status {
        Some(s) => status_to_string(s),
        None => "never executed".to_string(),
    };
    let mut desc = format!("{} ({})", &msg, exit);

    if let Some(out) = output {
        match str::from_utf8(&out.stdout) {
            Ok(s) if !s.trim().is_empty() => {
                desc.push_str("\n--- stdout\n");
                desc.push_str(s);
            }
            Ok(..) | Err(..) => {}
        }
        match str::from_utf8(&out.stderr) {
            Ok(s) if !s.trim().is_empty() => {
                desc.push_str("\n--- stderr\n");
                desc.push_str(s);
            }
            Ok(..) | Err(..) => {}
        }
    }

    return ProcessError {
        desc: desc,
        exit: status.cloned(),
        output: output.cloned(),
    };

    #[cfg(unix)]
    fn status_to_string(status: &ExitStatus) -> String {
        use std::os::unix::process::*;
        use libc;

        if let Some(signal) = status.signal() {
            let name = match signal as libc::c_int {
                libc::SIGABRT => ", SIGABRT: process abort signal",
                libc::SIGALRM => ", SIGALRM: alarm clock",
                libc::SIGFPE => ", SIGFPE: erroneous arithmetic operation",
                libc::SIGHUP => ", SIGHUP: hangup",
                libc::SIGILL => ", SIGILL: illegal instruction",
                libc::SIGINT => ", SIGINT: terminal interrupt signal",
                libc::SIGKILL => ", SIGKILL: kill",
                libc::SIGPIPE => ", SIGPIPE: write on a pipe with no one to read",
                libc::SIGQUIT => ", SIGQUIT: terminal quite signal",
                libc::SIGSEGV => ", SIGSEGV: invalid memory reference",
                libc::SIGTERM => ", SIGTERM: termination signal",
                libc::SIGBUS => ", SIGBUS: access to undefined memory",
                #[cfg(not(target_os = "haiku"))]
                libc::SIGSYS => ", SIGSYS: bad system call",
                libc::SIGTRAP => ", SIGTRAP: trace/breakpoint trap",
                _ => "",
            };
            format!("signal: {}{}", signal, name)
        } else {
            status.to_string()
        }
    }

    #[cfg(windows)]
    fn status_to_string(status: &ExitStatus) -> String {
        status.to_string()
    }
}

pub fn internal<S: fmt::Display>(error: S) -> CommandoError {
    _internal(&error)
}

fn _internal(error: &fmt::Display) -> CommandoError {
    CommandoError::from_kind(error.to_string().into()).into_internal()
}
