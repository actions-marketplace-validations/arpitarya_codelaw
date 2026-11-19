---
id: py/ambiguous-unicode-character-docstring
title: "No ambiguous Unicode character in a docstring"
languages: [python]
fix: suggest
fix_reason: "the ASCII character the table names is offered as the replacement; whether a docstring meant the other character (a name in another language, a typographic quote written on purpose) is the author's to say"
options: { allowed_confusables: { type: "string[]", default: [], description: "characters never reported: the ones the project writes on purpose" } }
stability: preview
ported_from: [ruff/RUF002]
since: 0.5.2
---

A docstring of a module, class or function holds no character that looks like an ASCII one but is not.

A character is reported when Unicode lists it as easy to mistake for one ASCII character. A letter or digit is judged with the word it is in: it is reported when the word has an ASCII character in it, or when every character of the word is confusable; a word with a non-ASCII character that is not confusable is a word of another script and is left alone. A character that is not a letter or digit (`’`, `×`, a no-break space) is always reported. An f-string is judged outside its replacement fields.

**The table** is the data package `@codelaw/data-confusables` / `codelaw-data-confusables` (generated from Unicode's `confusables.txt`, Unicode License v3; L6 decision 15), read only when a file has a non-ASCII character in a docstring. Without the package the rule reports nothing. Upstream's table is its own hand-kept list; this one is Unicode's, so it also reports characters such as `×` and the dotless `ı`, and it leaves out a character whose Unicode prototype is several characters (`“` and `”` are two apostrophes, `…` three full stops, `œ` and `½` a ligature and a fraction), which is not one character mistaken for another. Unicode also lists the box-drawing verticals `│` and `┃` as `l` and the en dash `–` as `-`: a project that draws tables or writes dashes on purpose lists them in `allowed_confusables`.

**Why:** A docstring is read by people, rendered by tools and quoted into examples that are run: a lookalike character in a name or an example makes the text lie about what it says.

```codelaw
native: py/ambiguous-unicode-character-docstring
```

## How to fix

1. Accept the suggestion where the ASCII character was meant: `codelaw fix --unsafe` writes it.
2. Where the other character is meant (a word in another language, a typographic quote), list it in `allowed_confusables`, or leave it.

## Examples

### Python

#### Don't

```python
def f():
    """Return the vаlue."""
```

#### Do

```python
def f():
    """Return the value."""
```
