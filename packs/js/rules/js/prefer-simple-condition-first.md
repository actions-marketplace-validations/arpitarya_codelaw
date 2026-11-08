---
id: js/prefer-simple-condition-first
title: "A bare name before a call in an && or || pair"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "moving the cheap test first can skip the call, which changes behaviour when the call has an effect"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-simple-condition-first]
since: 0.5.0
---

In `check() && ready`, the call runs even when `ready` is false. Putting the plain name first — `ready && check()` — reads the cheap test first and can skip the expensive one. This rule handles a pair whose left side is a call and whose right side is a name or a negated name; upstream also reorders longer chains and comparisons.

**Why:** Simple tests first make a condition quicker to read and to evaluate.

```codelaw
match:
  kind: binary_expression
  not: { inside: { kind: binary_expression } }
  any:
    - pattern: "$C && $S"
    - pattern: "$C || $S"
  has: { field: operator, pattern: $$OP }
constraints:
  C: { kind: call_expression }
  S: { any: [{ kind: identifier, not: { regex: '^(undefined|NaN|Infinity)$' } }, { kind: unary_expression, regex: '^![A-Za-z_$][A-Za-z0-9_$]*$' }] }
message: "Test `$S` first"
fix: "$S $OP $C"
```

## Examples

### JavaScript

#### Don't

```js
if (isValid(form) && enabled) {}
```

#### Do

```js
if (enabled && isValid(form)) {}
```


### JSX

#### Don't

```jsx
if (isValid(form) && enabled) {}
```

#### Do

```jsx
if (enabled && isValid(form)) {}
```


### TypeScript

#### Don't

```ts
if (isValid(form) && enabled) {}
```

#### Do

```ts
if (enabled && isValid(form)) {}
```


### TSX

#### Don't

```tsx
if (isValid(form) && enabled) {}
```

#### Do

```tsx
if (enabled && isValid(form)) {}
```
