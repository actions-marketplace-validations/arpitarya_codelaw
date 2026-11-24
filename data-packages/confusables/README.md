# @codelaw/data-confusables · codelaw-data-confusables

codelaw's confusables table: for each non-ASCII character Unicode lists as easy to mistake for one ASCII character (Cyrillic `а` for `a`, `’` for `'`, `×` for `x`), the ASCII character and the two names. `py/ambiguous-unicode-character-string`, `-docstring` and `-comment` (Ruff's RUF001–RUF003) read it to say *"did you mean `a`?"*.

- **Optional.** Neither `codelaw` package depends on it. Without it the three rules report nothing.
- **Data only.** `data/confusables.json` (schema 1), this README and its `NOTICE` (the Unicode License v3); no code, no dependency.
- **Generated, not written by hand:** `scripts/data-confusables-check.py --refresh` builds the table from Unicode's `confusables.txt` (UTS #39): every single-code-point, non-ASCII source whose prototype is one printable ASCII character. The package's `generated_from` names the version, the versioned URL and the file's SHA-256.
- **Its own version line:** the date of the data's last change, `YYYY.M.D` (`version` in the JSON). The engine puts it in the rules' cache question, so a refreshed table re-judges the files.
- **Kept current** by `.github/workflows/data-confusables.yml`, a weekly job that opens one pull request with the regenerated table.

Install: `npm install -D @codelaw/data-confusables` or `pip install codelaw-data-confusables`.
