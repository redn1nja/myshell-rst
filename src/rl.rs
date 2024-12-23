use std::borrow::Cow;
use std::borrow::Cow::{Borrowed, Owned};
use std::process::exit;
use rustyline::completion::FilenameCompleter;
use rustyline::{CompletionType, Config, Editor};
use rustyline::error::ReadlineError;
use rustyline::highlight::{CmdKind, Highlighter, MatchingBracketHighlighter};
use rustyline::hint::HistoryHinter;
use rustyline::history::DefaultHistory;
use rustyline::validate::MatchingBracketValidator;
use rustyline_derive::{Completer, Helper, Hinter, Validator};
#[derive(Helper, Completer, Hinter, Validator)]
pub struct MyHelper {
    #[rustyline(Completer)]
    completer: FilenameCompleter,
    highlighter: MatchingBracketHighlighter,
    #[rustyline(Validator)]
    validator: MatchingBracketValidator,
    #[rustyline(Hinter)]
    hinter: HistoryHinter,
    pub(crate) colored_prompt: String,
}

impl Highlighter for MyHelper {
    fn highlight<'l>(&self, line: &'l str, pos: usize) -> Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> Cow<'b, str> {
        if default {
            Borrowed(&self.colored_prompt)
        } else {
            Borrowed(prompt)
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned(hint.to_owned())
    }

    fn highlight_char(&self, line: &str, pos: usize, kind: CmdKind) -> bool {
        self.highlighter.highlight_char(line, pos, kind)
    }
}

pub fn create_rl() -> rustyline::Result<Editor<MyHelper, DefaultHistory>> {
    let config = Config::builder()
        .history_ignore_space(true)
        .auto_add_history(true)
        .completion_type(CompletionType::List)
        .build();
    let h = MyHelper {
        completer: FilenameCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        hinter: HistoryHinter::new(),
        colored_prompt: "".to_owned(),
        validator: MatchingBracketValidator::new(),
    };
    let mut rl = rustyline::Editor::with_config(config)?;
    rl.set_helper(Some(h));
    Ok(rl)
}

pub fn parse_input(rl: &mut Editor<MyHelper, DefaultHistory>) -> String {
    let mut prompt = std::env::current_dir().unwrap().display().to_string();
    prompt.push_str(" $ ");
    rl.helper_mut().unwrap().colored_prompt = format!("{prompt}");
    match rl.readline(&prompt) {
        Ok(line) => {
            rl.add_history_entry(line.as_str()).expect("");
            line
        },
        Err(ReadlineError::Eof) => {
            println!("CTRL-D");
            exit(1);
        },
        Err(ReadlineError::Interrupted) => {
            "".to_string()
        }
        Err(err) => {
            println!("Error: {:?}", err);
            exit(1);
        }
    }.trim().to_string()
}