//! Ruff's `flake8-executable` rows that read the file's executable bit
//! (W-184): `EXE001` and `EXE002`. The bit comes from
//! [`FileContext::is_executable`]; where there is none (off Unix, a file
//! checked alone) neither rule reports anything.

use crate::{NativeContext, NativeFinding, NativeRule};
use codelaw_core::Language;
use codelaw_lang::SyntaxNode;

const PY: &[Language] = &[Language::Python];

fn has_shebang(source: &str) -> bool {
    source.starts_with("#!")
}

/// `EXE001`: a `#!` first line in a file that is not executable.
pub struct ShebangNotExecutable;

impl NativeRule for ShebangNotExecutable {
    fn id(&self) -> &'static str {
        "py/shebang-not-executable"
    }
    fn languages(&self) -> &'static [Language] {
        PY
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["module"]
    }
    fn check(&self, _node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        // Ask only when there is a shebang: a file without one costs no `stat`.
        if !has_shebang(ctx.source) || ctx.file.is_executable() != Some(false) {
            return;
        }
        let end = ctx.source.find(['\n', '\r']).unwrap_or(ctx.source.len());
        out.push(NativeFinding { start: 0, end, message: "The shebang is present but the file is not executable".to_owned(), edits: Vec::new() });
    }
}

/// `EXE002`: an executable file with no `#!` first line.
pub struct ShebangMissingExecutableFile;

impl NativeRule for ShebangMissingExecutableFile {
    fn id(&self) -> &'static str {
        "py/shebang-missing-executable-file"
    }
    fn languages(&self) -> &'static [Language] {
        PY
    }
    fn kinds(&self) -> &'static [&'static str] {
        &["module"]
    }
    fn check(&self, _node: SyntaxNode<'_>, ctx: &NativeContext<'_>, out: &mut Vec<NativeFinding>) {
        if has_shebang(ctx.source) || ctx.file.is_executable() != Some(true) {
            return;
        }
        out.push(NativeFinding { start: 0, end: 0, message: "The file is executable but has no shebang".to_owned(), edits: Vec::new() });
    }
}

pub static RULES: &[&dyn NativeRule] = &[&ShebangNotExecutable, &ShebangMissingExecutableFile];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FileContext;
    use std::collections::BTreeMap;

    struct Moded(Option<bool>, Vec<u8>);
    impl FileContext for Moded {
        fn path(&self) -> &str {
            "run.py"
        }
        fn bytes(&self) -> &[u8] {
            &self.1
        }
        fn exists(&self, _: &str) -> bool {
            true
        }
        fn is_dir(&self, _: &str) -> bool {
            false
        }
        fn read(&self, _: &str) -> Option<String> {
            None
        }
        fn list(&self, _: &str) -> Option<Vec<String>> {
            None
        }
        fn is_executable(&self) -> Option<bool> {
            self.0
        }
    }

    fn run(rule: &dyn NativeRule, src: &str, bit: Option<bool>) -> Vec<NativeFinding> {
        let root = codelaw_lang::parse(Language::Python, src);
        let options = BTreeMap::new();
        let data = BTreeMap::new();
        let file = Moded(bit, src.as_bytes().to_vec());
        let ctx = NativeContext { source: src, language: Language::Python, semantic: None, options: &options, data: &data, file: &file };
        let mut out = Vec::new();
        for node in root.root().dfs() {
            if rule.kinds().contains(&node.kind().as_ref()) {
                rule.check(node, &ctx, &mut out);
            }
        }
        out
    }

    #[test]
    fn the_bit_and_the_shebang_agree_or_the_rules_report() {
        let sh = "#!/usr/bin/env python3\nprint(1)\n";
        let plain = "print(1)\n";
        assert_eq!(run(&ShebangNotExecutable, sh, Some(false)).len(), 1);
        assert!(run(&ShebangNotExecutable, sh, Some(true)).is_empty());
        assert!(run(&ShebangNotExecutable, plain, Some(false)).is_empty());
        assert_eq!(run(&ShebangMissingExecutableFile, plain, Some(true)).len(), 1);
        assert!(run(&ShebangMissingExecutableFile, sh, Some(true)).is_empty());
        assert!(run(&ShebangMissingExecutableFile, plain, Some(false)).is_empty());
    }

    #[test]
    fn no_bit_to_read_reports_nothing() {
        assert!(run(&ShebangNotExecutable, "#!/usr/bin/env python3\n", None).is_empty());
        assert!(run(&ShebangMissingExecutableFile, "print(1)\n", None).is_empty());
    }
}
