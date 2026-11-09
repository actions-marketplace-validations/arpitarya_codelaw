# @codelaw/data-polyfills · codelaw-data-polyfills

codelaw's polyfill table: for each core-js feature, the first version of each engine that has it built in, and which core-js entry (`core-js/es/array/flat`, `core-js/actual/…`) stands for which features. `js/no-unnecessary-polyfills` reads it to tell an import of a polyfill the project's targets no longer need.

- **Optional.** Neither `codelaw` package depends on it. Without it the rule reports nothing.
- **Data only.** `data/polyfills.json` (schema 1), this README and its `NOTICE`; no code, no dependency.
- **Generated, not written by hand:** `scripts/data-polyfills-check.py --refresh` builds the table from core-js-compat's published `data.json` and `entries.json` (MIT; the package version and tarball integrity are in `generated_from`). Nothing of core-js-compat is installed or run.
- **Its own version line:** the date of the data's last change, `YYYY.M.D` (`version` in the JSON). The engine puts it in the rule's cache question, so a refreshed table re-judges the files.
- **Kept current** by `.github/workflows/data-polyfills.yml`, a weekly job that opens one pull request with the regenerated table.

Install: `npm install -D @codelaw/data-polyfills` or `pip install codelaw-data-polyfills`.
