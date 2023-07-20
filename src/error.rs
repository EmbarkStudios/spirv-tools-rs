use spirv_tools_sys::{diagnostics, shared};

pub use diagnostics::MessageLevel;
pub use shared::SpirvResult;

#[derive(Debug, PartialEq)]
pub struct Error {
    pub inner: shared::SpirvResult,
    pub diagnostic: Option<Diagnostic>,
}

use std::fmt;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.diagnostic {
            Some(diag) => {
                f.write_fmt(format_args!(
                    "error:{}:{} - {}",
                    diag.line, diag.column, diag.message
                ))?;

                if !diag.notes.is_empty() {
                    f.write_fmt(format_args!("\n{}", diag.notes))?;
                }

                Ok(())
            }
            None => f.write_str("an unknown error occurred"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    pub line: usize,
    pub column: usize,
    pub index: usize,
    pub message: String,
    pub notes: String,
    pub is_text: bool,
}

#[cfg(feature = "use-compiled-tools")]
impl Diagnostic {
    pub(crate) unsafe fn from_diag(
        diag: *mut diagnostics::Diagnostic,
    ) -> Result<Self, shared::SpirvResult> {
        if diag.is_null() {
            return Err(shared::SpirvResult::Success);
        }

        let (message, notes) = Message::message_and_notes_from_cstr((*diag).error);

        let res = Self {
            line: (*diag).position.line,
            column: (*diag).position.column,
            index: (*diag).position.index,
            message,
            notes,
            is_text: (*diag).is_text_source,
        };

        diagnostics::diagnostic_destroy(diag);
        Ok(res)
    }
}

impl From<String> for Diagnostic {
    fn from(message: String) -> Self {
        Self {
            line: 0,
            column: 0,
            index: 0,
            is_text: false,
            message,
            notes: String::new(),
        }
    }
}

impl From<Message> for Diagnostic {
    fn from(msg: Message) -> Self {
        Self {
            line: msg.line,
            column: msg.column,
            index: msg.index,
            message: msg.message,
            notes: msg.notes,
            is_text: false,
        }
    }
}

#[derive(Debug)]
pub struct Message {
    pub level: MessageLevel,
    pub source: Option<String>,
    pub line: usize,
    pub column: usize,
    pub index: usize,
    pub message: String,
    /// Some messages can include additional information, typically instructions
    pub notes: String,
}

impl Message {
    #[cfg(feature = "use-installed-tools")]
    pub(crate) fn fatal(message: String) -> Self {
        Self {
            level: MessageLevel::Fatal,
            source: None,
            line: 0,
            column: 0,
            index: 0,
            message,
            notes: String::new(),
        }
    }

    #[cfg(feature = "use-compiled-tools")]
    unsafe fn message_and_notes_from_cstr(msg: *const std::os::raw::c_char) -> (String, String) {
        let full_message = std::ffi::CStr::from_ptr(msg).to_string_lossy();

        if let Some(ind) = full_message.find('\n') {
            (
                full_message[..ind].to_owned(),
                full_message[ind + 1..].to_owned(),
            )
        } else {
            (full_message.into_owned(), String::new())
        }
    }

    #[cfg(feature = "use-compiled-tools")]
    pub(crate) fn from_parts(
        level: MessageLevel,
        source: *const std::os::raw::c_char,
        source_pos: *const diagnostics::Position,
        msg: *const std::os::raw::c_char,
    ) -> Self {
        unsafe {
            let source = if source.is_null() {
                None
            } else {
                Some(std::ffi::CStr::from_ptr(source).to_string_lossy())
            };

            let (message, notes) = Self::message_and_notes_from_cstr(msg);

            let (line, column, index) = if source_pos.is_null() {
                (0, 0, 0)
            } else {
                (
                    (*source_pos).line,
                    (*source_pos).column,
                    (*source_pos).index,
                )
            };

            Self {
                level,
                source: source.and_then(|source| {
                    if source.is_empty() {
                        None
                    } else {
                        Some(source.into_owned())
                    }
                }),
                line,
                column,
                index,
                message,
                notes,
            }
        }
    }

    #[cfg(feature = "use-installed-tools")]
    pub(crate) fn parse(s: &str) -> Option<Self> {
        s.find(": ")
            .and_then(|i| {
                let level = match &s[..i] {
                    "error" => MessageLevel::Error,
                    "warning" => MessageLevel::Warning,
                    "info" => MessageLevel::Info,
                    _ => return None,
                };

                Some((level, i))
            })
            .and_then(|(level, i)| {
                s[i + 7..]
                    .find(": ")
                    .and_then(|i2| {
                        s[i + 7..i + 7 + i2]
                            .parse::<usize>()
                            .ok()
                            .map(|index| (index, i2))
                    })
                    .map(|(index, i2)| (level, index, i + 7 + i2 + 2))
            })
            .map(|(level, index, last)| Self {
                level,
                index,
                message: s[last..].to_owned(),
                source: None,
                line: 0,
                column: 0,
                notes: String::new(),
            })
    }
}

pub trait MessageCallback {
    fn on_message(&mut self, msg: Message);
}

impl<F> MessageCallback for F
where
    F: FnMut(Message),
{
    fn on_message(&mut self, msg: Message) {
        self(msg);
    }
}
