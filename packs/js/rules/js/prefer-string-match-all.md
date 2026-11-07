---
id: js/prefer-string-match-all
title: "for…of over matchAll() rather than a while loop over exec()"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`matchAll` needs a global (`g`) regular expression and starts from index 0 whatever `lastIndex` was; the `let` declaration of the match variable is left for the reviewer to drop"
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-string-match-all]
since: 0.5.0
---

`while ((match = regex.exec(text)) !== null) { … }` collects every match by hand. `for (const match of text.matchAll(regex)) { … }` does the same without the assignment in the condition or the `lastIndex` state.

**Why:** The loop says it visits every match; the `exec` form makes the reader check it terminates.

```codelaw
match:
  kind: while_statement
  any:
    - pattern: "while (($M = $R.exec($S)) !== null) $B"
    - pattern: "while (($M = $R.exec($S)) != null) $B"
    - pattern: "while ($M = $R.exec($S)) $B"
constraints:
  M: { kind: identifier }
  R: { any: [{ kind: identifier }, { kind: regex, regex: '/[a-z]*g[a-z]*$' }] }
message: "Iterate the matches with `matchAll`"
fix: "for (const $M of $S.matchAll($R)) $B"
```

## Examples

### JavaScript

#### Don't

```js
let match;
while ((match = pattern.exec(text)) !== null) {
  found.push(match[0]);
}
```

#### Do

```js
for (const match of text.matchAll(pattern)) {
  found.push(match[0]);
}
```

#### Fixed

```js
let match;
for (const match of text.matchAll(pattern)) {
  found.push(match[0]);
}
```


### JSX

#### Don't

```jsx
let match;
while ((match = pattern.exec(text)) !== null) {
  found.push(match[0]);
}
```

#### Do

```jsx
for (const match of text.matchAll(pattern)) {
  found.push(match[0]);
}
```

#### Fixed

```jsx
let match;
for (const match of text.matchAll(pattern)) {
  found.push(match[0]);
}
```


### TypeScript

#### Don't

```ts
let match;
while ((match = pattern.exec(text)) !== null) {
  found.push(match[0]);
}
```

#### Do

```ts
for (const match of text.matchAll(pattern)) {
  found.push(match[0]);
}
```

#### Fixed

```ts
let match;
for (const match of text.matchAll(pattern)) {
  found.push(match[0]);
}
```


### TSX

#### Don't

```tsx
let match;
while ((match = pattern.exec(text)) !== null) {
  found.push(match[0]);
}
```

#### Do

```tsx
for (const match of text.matchAll(pattern)) {
  found.push(match[0]);
}
```

#### Fixed

```tsx
let match;
for (const match of text.matchAll(pattern)) {
  found.push(match[0]);
}
```
