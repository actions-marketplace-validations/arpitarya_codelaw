# @codelaw/provider-tailwind

Builds the class table of codelaw's `tailwind` pack
([`@codelaw/rules-tailwind`](https://www.npmjs.com/package/@codelaw/rules-tailwind))
from **the Tailwind CSS installed in your project** — its version, its
stylesheet's `@theme` and `@utility`, its plugins. codelaw runs it with Node when
the pack is installed and caches the table until Tailwind, the stylesheet or this
package changes. It holds no copy of Tailwind; `tailwindcss` (v4) is its peer
dependency.

You do not call it: `@codelaw/rules-tailwind` depends on it, and `codelaw init`
installs both. Point it at your stylesheet when it is not found on its own:

```toml
# codelaw.toml
[providers.tailwind]
stylesheet = "src/app.css"
```

Without it, the tailwind rules that need the table and the formatter's class
sorting are skipped with a note; `[providers] on_missing = "error"` stops the run
instead. MIT licence.
