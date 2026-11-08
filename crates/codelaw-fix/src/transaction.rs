//! Writing a multi-file fix all or nothing (SR-MULTI-FILE-FIXES decision
//! 2, safeguard 3): every file goes to a temporary file beside it first,
//! each is checked unchanged since it was read, and only then is each
//! renamed into place. A rename that fails puts back every file already
//! renamed, so the files hold the whole fix or none of it.

use codelaw_cache::hash_bytes;
use std::path::{Path, PathBuf};

/// One file of a multi-file fix: where it is, the bytes it held when the
/// fix was computed, and the bytes to write.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileWrite {
    pub path: PathBuf,
    pub read: Vec<u8>,
    pub bytes: Vec<u8>,
}

/// Why nothing was written.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WriteAllError {
    /// A file changed on disk since it was read (SR-ARCHITECTURE decision 12).
    Changed(PathBuf),
    /// A write or a rename failed; every file holds what it held before.
    Failed(PathBuf, String),
    /// A rename failed and a file already renamed could not be put back:
    /// the files named hold the fix, the others don't.
    Partial(Vec<PathBuf>, String),
}

impl WriteAllError {
    pub fn note(&self) -> String {
        match self {
            WriteAllError::Changed(p) => format!("{} changed on disk during the run, so no file of the fix was written", p.display()),
            WriteAllError::Failed(p, e) => format!("{} could not be written ({e}), so no file of the fix was written", p.display()),
            WriteAllError::Partial(ps, e) => format!(
                "a rename failed ({e}) and {} could not be put back: check {}",
                if ps.len() == 1 { "one file" } else { "some files" },
                ps.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(", ")
            ),
        }
    }
}

/// Write every file or none (safeguard 3).
pub fn write_all_or_nothing(writes: &[FileWrite]) -> Result<(), WriteAllError> {
    write_with(writes, &mut |from, to| std::fs::rename(from, to))
}

/// [`write_all_or_nothing`] with the rename supplied, so a test can make
/// one fail.
pub fn write_with(writes: &[FileWrite], rename: &mut dyn FnMut(&Path, &Path) -> std::io::Result<()>) -> Result<(), WriteAllError> {
    use std::io::Write;
    // Every target through its symlinks, and a temporary file beside it.
    let mut staged: Vec<(PathBuf, PathBuf, &FileWrite)> = Vec::with_capacity(writes.len());
    let clean = |staged: &[(PathBuf, PathBuf, &FileWrite)]| {
        for (_, tmp, _) in staged {
            let _ = std::fs::remove_file(tmp);
        }
    };
    for (i, w) in writes.iter().enumerate() {
        let target = match std::fs::canonicalize(&w.path) {
            Ok(t) => t,
            Err(e) => {
                clean(&staged);
                return Err(WriteAllError::Failed(w.path.clone(), e.to_string()));
            }
        };
        let (Some(dir), Some(name)) = (target.parent(), target.file_name()) else {
            clean(&staged);
            return Err(WriteAllError::Failed(w.path.clone(), "no parent folder".into()));
        };
        let tmp = dir.join(format!(".{}.codelaw-{}-{i}.tmp", name.to_string_lossy(), std::process::id()));
        let written = std::fs::File::create(&tmp).and_then(|mut f| {
            f.write_all(&w.bytes)?;
            f.sync_all()
        });
        if let Err(e) = written {
            let _ = std::fs::remove_file(&tmp);
            clean(&staged);
            return Err(WriteAllError::Failed(w.path.clone(), e.to_string()));
        }
        if let Ok(meta) = std::fs::metadata(&target) {
            let _ = std::fs::set_permissions(&tmp, meta.permissions());
        }
        staged.push((target, tmp, w));
    }
    // Every file still holds what the fix was computed from, checked last
    // so the window for a lost save is as small as it can be.
    for (target, _, w) in &staged {
        match std::fs::read(target) {
            Ok(now) if hash_bytes(&now) == hash_bytes(&w.read) => {}
            Ok(_) => {
                clean(&staged);
                return Err(WriteAllError::Changed(w.path.clone()));
            }
            Err(e) => {
                clean(&staged);
                return Err(WriteAllError::Failed(w.path.clone(), e.to_string()));
            }
        }
    }
    for (i, (target, tmp, w)) in staged.iter().enumerate() {
        if let Err(e) = rename(tmp, target) {
            clean(&staged[i..]);
            // Put back every file already renamed.
            let mut stuck = Vec::new();
            for (t, _, before) in &staged[..i] {
                if restore(t, &before.read).is_err() {
                    stuck.push(before.path.clone());
                }
            }
            return Err(if stuck.is_empty() { WriteAllError::Failed(w.path.clone(), e.to_string()) } else { WriteAllError::Partial(stuck, e.to_string()) });
        }
    }
    let mut dirs: Vec<&Path> = staged.iter().filter_map(|(t, _, _)| t.parent()).collect();
    dirs.sort();
    dirs.dedup();
    for d in dirs {
        if let Ok(f) = std::fs::File::open(d) {
            let _ = f.sync_all();
        }
    }
    Ok(())
}

/// Put a file's bytes back, through a temporary file and a rename.
fn restore(target: &Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::io::Write;
    let dir = target.parent().ok_or_else(|| std::io::Error::other("no parent folder"))?;
    let name = target.file_name().ok_or_else(|| std::io::Error::other("no file name"))?.to_string_lossy();
    let tmp = dir.join(format!(".{name}.codelaw-{}-restore.tmp", std::process::id()));
    let mut f = std::fs::File::create(&tmp)?;
    f.write_all(bytes)?;
    f.sync_all()?;
    std::fs::rename(&tmp, target)
}
