---
id: js/future-reserved-words
title: "No variable named after a reserved word"
languages: [javascript, jsx]
fix: manual
fix_reason: "the new name is the author's choice"
stability: preview
ported_from: [eslint-plugin-sonarjs/future-reserved-words]
since: 0.5.0
---

A variable, function or parameter is not named after a word the language reserves (`implements`, `interface`, `package`, `private`, `protected`, `public`, `static`, `enum`, `await`, `yield`, `let`).

**Why:** The name is a syntax error in strict mode and in modules.

```codelaw

match:
  kind: identifier
  regex: '^(implements|interface|package|private|protected|public|static|enum|await|yield|let)$'
  any:
    - inside: { kind: variable_declarator, field: name }
    - inside: { kind: function_declaration, field: name }
    - inside: { kind: formal_parameters }
message: "`$$MATCH` is reserved in strict mode or modules; rename it"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
var package = load();
```

#### Do

```js
var pkg = load();
```

### JSX

#### Don't

```jsx
var package = load();
```

#### Do

```jsx
var pkg = load();
```
