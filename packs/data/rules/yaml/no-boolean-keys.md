---
id: yaml/no-boolean-keys
title: "No unquoted key that YAML 1.1 reads as a boolean"
languages: [yaml]
fix: suggest
fix_reason: "quoting the key keeps it a string in every loader; a key meant as a boolean is written `true` or `false`"
stability: preview
ported_from: [biome/nursery/noYamlBooleanKeys]
since: 0.5.1
---

A mapping key that is an unquoted `yes`, `no`, `on`, `off`, `true` or `false` (in any of the capitalisations YAML 1.1 reads as a boolean) is reported. The suggestion quotes it. GitHub Actions' `on:` is the well-known one; a project that writes it turns the rule off for workflows. `yaml/no-ambiguous-booleans` is the same check on values. Behaviour of Biome's `noYamlBooleanKeys`.

**Why:** YAML 1.1 loaders (PyYAML, Ruby's Psych) turn `no:` into the key `false`, so `country: no` style data silently changes its keys.

```codelaw
match:
  any:
    - kind: block_mapping_pair
    - kind: flow_pair
  has:
    field: key
    pattern: $K
    regex: '^(yes|Yes|YES|no|No|NO|on|On|ON|off|Off|OFF|true|True|TRUE|false|False|FALSE)$'
message: "The key `$K` is a boolean to YAML 1.1 loaders; quote it"
fix:
  - replace: { target: $K, with: '"$K"' }
```

## Examples

### YAML

#### Don't

```yaml
no: Norway
yes: Yemen
```

#### Do

```yaml
"no": Norway
"yes": Yemen
```

#### Fixed

```yaml
"no": Norway
"yes": Yemen
```
