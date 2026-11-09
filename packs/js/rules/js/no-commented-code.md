---
id: js/no-commented-code
title: "No code switched off inside a comment"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "deleting a comment loses what it held, which version control keeps but the author may not want gone; the diff is for them to accept"
stability: preview
ported_from: [eslint-plugin-sonarjs/no-commented-code]
since: 0.5.0
---

A block comment, or a run of `//` comments on adjacent lines, whose text is JavaScript — it parses with no errors, has at least one statement, and has a line that reads as code (ends with `;`, `{` or `}`, or holds an operator like `===`, `&&`, `=>`, or starts with `const`, `if (`, `return …`) — is reported over its whole span. The report skips a lone `}`, text that starts `e.g.`, `for example` or `example:`, and a single statement that is only a name, a string or number, a unary `+`/`-`, a `break`/`continue`/label, a bare `return`/`throw` (or of a name), or an expression with no closing `;` (`// foo()` is a note, `// foo();` is code). A comment that is only attributes of a JSX element counts when it does not parse as a statement. Behaviour of eslint-plugin-sonarjs 4.2.2 (read, not copied); its footprint-based line recogniser is replaced by the checks above.

**Why:** Code left in comments goes stale unseen, gets read as if it mattered, and is kept by version control anyway.

```codelaw
native: js/no-commented-code
```

## Examples

### JavaScript

#### Don't

```js
const total = price * count;
// const taxed = total * 1.2;
report(total);
```

#### Do

```js
const total = price * count;
report(total);
```

#### Fixed

```js
const total = price * count;
report(total);
```


### JSX

#### Don't

```jsx
const total = price * count;
// const taxed = total * 1.2;
report(total);
```

#### Do

```jsx
const total = price * count;
report(total);
```

#### Fixed

```jsx
const total = price * count;
report(total);
```


### TypeScript

#### Don't

```ts
const total = price * count;
// const taxed = total * 1.2;
report(total);
```

#### Do

```ts
const total = price * count;
report(total);
```

#### Fixed

```ts
const total = price * count;
report(total);
```


### TSX

#### Don't

```tsx
const total = price * count;
// const taxed = total * 1.2;
report(total);
```

#### Do

```tsx
const total = price * count;
report(total);
```

#### Fixed

```tsx
const total = price * count;
report(total);
```
