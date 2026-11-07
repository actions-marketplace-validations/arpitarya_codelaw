---
id: yaml/no-undeclared-alias
title: "Every alias refers to an anchor defined before it"
languages: [yaml]
fix: suggest
fix_reason: "the anchors defined before the alias within two edits are choices, nearest first; an alias whose anchor comes later, or with none near, is manual"
fix_exception: restructure
stability: preview
since: 0.1.0
---

An alias `*name` refers to an anchor `&name` defined earlier in the same document.

**Why:** An alias with no anchor before it is an error: YAML loaders refuse the whole file, and anchors don't carry over from one document to the next.

```codelaw
native: yaml/no-undeclared-alias
```

## How to fix

1. Find the anchor the alias meant.
2. Move the anchored node above the alias, fix the spelling, or add the anchor.

## Examples

### YAML

#### Don't

```yaml
defaults: &defaults
  port: 443
production:
  <<: *default
  host: example.com
```

#### Fixed

```yaml
defaults: &defaults
  port: 443
production:
  <<: *defaults
  host: example.com
```

#### Do

```yaml
defaults: &defaults
  port: 443
production:
  <<: *defaults
  host: example.com
```

