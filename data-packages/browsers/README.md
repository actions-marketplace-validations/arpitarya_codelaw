# @codelaw/data-browsers · codelaw-data-browsers

codelaw's browser-support table: for each CSS feature a fix may write, the first version of each browser that supports it without a vendor prefix. The CSS rules with `fix_target: { browsers: … }` read it to decide whether their fix is automatic for a project's `[targets] browsers`.

- **Optional.** Neither `codelaw` package depends on it. Without it, those fixes stay suggestions.
- **Data only.** `data/browsers.json` (schema 1), this README and its `NOTICE`; no code, no dependency.
- **Its own version line:** the date of the data's last change, `YYYY.M.D` (`version` in the JSON). `codelaw --version --json` names the version a project uses.
- **Every row cites its sources** — vendor release notes, feature status or bug tracker, or the specification. Nothing comes from caniuse or MDN.
- **Kept current** by `.github/workflows/data-browsers.yml`, a weekly check that opens a pull request naming the rows to review.

Install: `npm install -D @codelaw/data-browsers` or `pip install codelaw-data-browsers`.
