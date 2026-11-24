# @codelaw/data-python-deprecations · codelaw-data-python-deprecations

codelaw's table of deprecated Python standard-library names: modules, classes, functions and methods, attributes, decorators and single arguments, each with the Python version that deprecates it and, where the documentation says, the version that removes it. `py/deprecated-module`, `py/deprecated-class`, `py/deprecated-method`, `py/deprecated-attribute` and `py/deprecated-argument` read it.

- **Optional.** Neither `codelaw` package depends on it. Without it the five rules report nothing.
- **Data only.** `data/python-deprecations.json` (schema 1), this README and its `NOTICE`; no code, no dependency.
- **Generated, not written by hand:** `scripts/data-python-deprecations-check.py --refresh` reads the `.. deprecated::` and `.. deprecated-removed::` directives and the `:deprecated:` module option of CPython's `Doc/library/*.rst` at a release tag (`generated_from` names the tag and its commit). Nothing of CPython is installed or run, and no documentation text is copied: a row is a name and two version numbers.
- **What a row is:** a name the documentation deprecates as a whole. A deprecation of one way of calling (`Passing x positionally is deprecated`) is not a row, and a deprecation of one named parameter is an `arguments` row. The PEP 585 notes on `typing.Dict` and the like are left to `py/deprecated-typing-alias`.
- **Its own version line:** the date of the data's last change, `YYYY.M.D` (`version` in the JSON). The engine puts it in the rules' cache question, so a refreshed table re-judges the files.
- **Kept current** by `.github/workflows/data-python-deprecations.yml`, a weekly job that opens one pull request with the regenerated table.

Install: `npm install -D @codelaw/data-python-deprecations` or `pip install codelaw-data-python-deprecations`.
