---
id: js/no-continue
title: "No continue"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "an unlabelled `continue` guarding the rest of a loop body is rewritten to a negated `if` around that rest; a body that declares a function, or any other `continue`, is not auto"
stability: preview
ported_from: [eslint/no-continue]
since: 0.5.0
---

Loops do not use `continue`.

**Why:** A `continue` jumps over the rest of the loop body; an `if` around it states the condition where it applies.

```codelaw
match:
  kind: continue_statement
  any:
    - inside:
        kind: if_statement
        inside:
          all:
            - { pattern: $BODY }
            - pattern: { context: "for (;;) { if ($C) continue; $$$R }", selector: statement_block }
            - inside: { field: body, any: [{kind: for_statement}, {kind: for_in_statement}, {kind: while_statement}, {kind: do_statement}] }
    - { kind: continue_statement }
auto_when:
  inside:
    kind: if_statement
    inside:
      kind: statement_block
      not: { has: { stopBy: end, any: [{kind: function_declaration}, {kind: generator_function_declaration}] } }
message: "Unexpected `continue`"
fix:
  - replace: { target: $BODY, with: "{\n  if (!($C)) {\n    $$$R\n  }\n}" }
```

## How to fix

1. Invert the condition and wrap the rest of the body in an `if`.

## Examples

### JavaScript

#### Don't

```js
for (const x of xs) {
  if (!x) continue;
  use(x);
}
```

#### Fixed

```js
for (const x of xs) {
  if (!(!x)) {
    use(x);
  }
}
```

#### Do

```js
for (const x of xs) {
  if (x) use(x);
}
```

### JSX

#### Don't

```jsx
for (const x of xs) {
  if (!x) continue;
  use(x);
}
```

#### Fixed

```jsx
for (const x of xs) {
  if (!(!x)) {
    use(x);
  }
}
```

#### Do

```jsx
for (const x of xs) {
  if (x) use(x);
}
```

### TypeScript

#### Don't

```ts
for (const x of xs) {
  if (!x) continue;
  use(x);
}
```

#### Fixed

```ts
for (const x of xs) {
  if (!(!x)) {
    use(x);
  }
}
```

#### Do

```ts
for (const x of xs) {
  if (x) use(x);
}
```

### TSX

#### Don't

```tsx
for (const x of xs) {
  if (!x) continue;
  use(x);
}
```

#### Fixed

```tsx
for (const x of xs) {
  if (!(!x)) {
    use(x);
  }
}
```

#### Do

```tsx
for (const x of xs) {
  if (x) use(x);
}
```
