---
id: css/no-invalid-property-init-value
title: "`initial-value` of a registered property matches its `syntax`"
languages: [css]
fix: manual
fix_reason: "whether the syntax or the initial value is the one that is wrong is the author's knowledge"
fix_exception: value
stability: preview
ported_from: [biome/nursery/noInvalidPropertyInitValue]
since: 0.5.1
---

In an `@property` rule, `initial-value` is a value its `syntax` descriptor accepts. The basic types are checked — `<number>`, `<integer>`, `<length>`, `<percentage>`, `<length-percentage>`, `<angle>`, `<time>`, `<resolution>`, `<color>`, `<string>`, `<url>`, `<custom-ident>` — with `+` and `#` lists, `|` alternatives and keywords; `*` and a type the rule does not know accept any value. Behaviour of Biome's `noInvalidPropertyInitValue`.

**Why:** A registered property whose initial value does not fit its syntax is invalid at parse time: the whole `@property` rule is dropped and the custom property stays unregistered.

```codelaw
native: css/no-invalid-property-init-value
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### CSS

#### Don't

```css
@property --size {
  syntax: "<length>";
  inherits: false;
  initial-value: red;
}
```

#### Do

```css
@property --size {
  syntax: "<length>";
  inherits: false;
  initial-value: 4px;
}
```
