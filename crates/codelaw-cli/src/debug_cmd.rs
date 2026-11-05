//! `codelaw debug changed-functions` (W-71 step 1): which functions the
//! change set adds, edits or leaves alone, by token stream against the base
//! revision. What the test-for-new-code rules will judge, made inspectable.
//!
//! `codelaw debug sort-classes` (W-145): the class sorting of SR-FORMATTER
//! decision 9 alone, with no layout, so `scripts/tailwind-parity.py` can
//! measure the sorter against prettier-plugin-tailwindcss on the same text.

use crate::{Common, engine_options, fail, load};
use clap::{Args, Subcommand};
use std::process::ExitCode;

#[derive(Args)]
pub struct DebugArgs {
    #[command(subcommand)]
    action: DebugAction,
}

#[derive(Subcommand)]
enum DebugAction {
    /// Each changed file's functions as new, edited or untouched (needs `--changed-since` or `--staged`).
    ChangedFunctions {
        /// text or json.
        #[arg(long, default_value = "text")]
        format: String,
        #[command(flatten)]
        common: Common,
    },
    /// Sort each file's class lists with the provider's table, nothing else: the sorted text, or with --write the files rewritten.
    SortClasses {
        files: Vec<std::path::PathBuf>,
        #[arg(long)]
        write: bool,
        #[command(flatten)]
        common: Common,
    },
}

fn sort_classes(files: Vec<std::path::PathBuf>, write: bool, common: Common) -> ExitCode {
    let options = match engine_options(&common) {
        Ok(o) => o,
        Err(e) => return fail(e),
    };
    let project = match load(&[], options) {
        Ok(p) => p,
        Err(e) => return fail(e),
    };
    let Some(sort) = project.formatting.options.class_sort.clone() else {
        return fail("no provider gave a class table here (the tailwind pack, its provider and node_modules/tailwindcss are needed)");
    };
    for f in files {
        let Some(language) = codelaw_core::Language::from_path(&f) else { continue };
        let Ok(text) = std::fs::read_to_string(&f) else { continue };
        let out = match codelaw_format::sort_class_lists(language, &text, &sort) {
            Ok(o) => o.unwrap_or(text),
            Err(e) => return fail(format!("{}: {e}", f.display())),
        };
        if write {
            if let Err(e) = std::fs::write(&f, out) {
                return fail(format!("{}: {e}", f.display()));
            }
        } else {
            print!("{out}");
        }
    }
    ExitCode::SUCCESS
}

pub fn debug(args: DebugArgs) -> ExitCode {
    let (format, common) = match args.action {
        DebugAction::ChangedFunctions { format, common } => (format, common),
        DebugAction::SortClasses { files, write, common } => return sort_classes(files, write, common),
    };
    if !matches!(format.as_str(), "text" | "json") {
        return fail(format!("unknown --format `{format}` (text or json)"));
    }
    let options = match engine_options(&common) {
        Ok(o) => o,
        Err(e) => return fail(e),
    };
    let project = match load(&[], options) {
        Ok(p) => p,
        Err(e) => return fail(e),
    };
    match project.changed_functions() {
        Ok(c) if format == "json" => {
            println!("{}", serde_json::to_string_pretty(&c.to_json()).expect("json"));
            ExitCode::SUCCESS
        }
        Ok(c) => {
            print!("{}", c.to_text());
            ExitCode::SUCCESS
        }
        Err(e) => fail(e),
    }
}
