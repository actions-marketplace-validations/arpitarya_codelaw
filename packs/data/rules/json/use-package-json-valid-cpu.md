---
id: json/use-package-json-valid-cpu
title: "`cpu` in package.json names real architectures"
languages: [json]
files: ["**/package.json"]
fix: manual
fix_reason: "which architecture was meant is the author's knowledge"
fix_exception: value
stability: preview
ported_from: [biome/nursery/usePackageJsonValidCpu, eslint-plugin-package-json/valid-cpu]
since: 0.5.1
---

`cpu` is an array whose entries are architectures `process.arch` can return — `arm`, `arm64`, `ia32`, `loong64`, `mips`, `mipsel`, `ppc`, `ppc64`, `riscv64`, `s390`, `s390x`, `x64` — each optionally negated with `!`. Behaviour of Biome's `usePackageJsonValidCpu`.

**Why:** npm installs a package only on the architectures `cpu` allows; a misspelt name makes it install nowhere or everywhere without a word.

```codelaw
target: [cpu, "cpu[*]"]
assert:
  any:
    - type: array
    - matches: '^!?(arm|arm64|ia32|loong64|mips|mipsel|ppc|ppc64|riscv64|s390|s390x|x64)$'
message: "`$VALUE` is not a CPU architecture npm knows"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JSON

#### Don't

```json path=package.json
{
  "name": "app",
  "cpu": ["x86_64"]
}
```

#### Do

```json path=package.json
{
  "name": "app",
  "cpu": ["x64"]
}
```
