---
id: js/prefer-string-pad-start-end
title: "padStart() and padEnd() rather than repeat() for padding"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "when the value is already longer than the width, `repeat` throws and `padStart` returns the value; the fix removes that error, and assumes the value is a string"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-string-pad-start-end]
since: 0.5.0
---

`'*'.repeat(10 - name.length) + name` is `name.padStart(10, '*')`, and `name + ' '.repeat(10 - name.length)` is `name.padEnd(10, ' ')`. The padding is one character, the width a number literal and the value a name, as upstream requires.

**Why:** The method says what is meant and does not throw for a value wider than the column.

```codelaw
match:
  kind: binary_expression
  any:
    - all:
        - pattern: "$P.repeat($N - $V.length) + $V"
        - pattern: $W1
    - all:
        - pattern: "$V + $P.repeat($N - $V.length)"
        - pattern: $W2
constraints:
  P: { kind: string, regex: '^(".(?:")|''.(?:''))$' }
  N: { kind: number }
  V: { kind: identifier }
message: "Pad with the string method"
fix:
  - replace: { target: $W1, with: "$V.padStart($N, $P)" }
  - replace: { target: $W2, with: "$V.padEnd($N, $P)" }
```

## Examples

### JavaScript

#### Don't

```js
const cell = "*".repeat(10 - name.length) + name;
```

#### Do

```js
const cell = name.padStart(10, "*");
```


### JSX

#### Don't

```jsx
const cell = "*".repeat(10 - name.length) + name;
```

#### Do

```jsx
const cell = name.padStart(10, "*");
```


### TypeScript

#### Don't

```ts
const cell = "*".repeat(10 - name.length) + name;
```

#### Do

```ts
const cell = name.padStart(10, "*");
```


### TSX

#### Don't

```tsx
const cell = "*".repeat(10 - name.length) + name;
```

#### Do

```tsx
const cell = name.padStart(10, "*");
```
