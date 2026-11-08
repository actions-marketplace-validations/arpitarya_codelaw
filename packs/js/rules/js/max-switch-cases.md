---
id: js/max-switch-cases
title: "No switch with more than 30 cases"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the table that replaces it is the author's design"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/max-switch-cases]
since: 0.5.0
---

A `switch` has at most 30 `case` clauses (SonarJS's default).

**Why:** A switch that long is a lookup table written as code; a map or an object says so and is easier to extend.

```codelaw
match:
  kind: switch_statement
  has:
    field: body
    has: { kind: switch_case, nthChild: { position: 31, ofRule: { kind: switch_case } } }
message: "This switch has more than 30 cases; use a lookup table"
```

## How to fix

1. Replace the cases with an object or a `Map` from each value to its result, and look the value up.

## Examples

### JavaScript

#### Don't

```js
switch (code) {
  case 1: return "a"; case 2: return "b"; case 3: return "c"; case 4: return "d"; case 5: return "e";
  case 6: return "f"; case 7: return "g"; case 8: return "h"; case 9: return "i"; case 10: return "j";
  case 11: return "k"; case 12: return "l"; case 13: return "m"; case 14: return "n"; case 15: return "o";
  case 16: return "p"; case 17: return "q"; case 18: return "r"; case 19: return "s"; case 20: return "t";
  case 21: return "u"; case 22: return "v"; case 23: return "w"; case 24: return "x"; case 25: return "y";
  case 26: return "z"; case 27: return "A"; case 28: return "B"; case 29: return "C"; case 30: return "D";
  case 31: return "E";
}
```

#### Do

```js
return LETTERS[code];
```


### JSX

#### Don't

```jsx
switch (code) {
  case 1: return "a"; case 2: return "b"; case 3: return "c"; case 4: return "d"; case 5: return "e";
  case 6: return "f"; case 7: return "g"; case 8: return "h"; case 9: return "i"; case 10: return "j";
  case 11: return "k"; case 12: return "l"; case 13: return "m"; case 14: return "n"; case 15: return "o";
  case 16: return "p"; case 17: return "q"; case 18: return "r"; case 19: return "s"; case 20: return "t";
  case 21: return "u"; case 22: return "v"; case 23: return "w"; case 24: return "x"; case 25: return "y";
  case 26: return "z"; case 27: return "A"; case 28: return "B"; case 29: return "C"; case 30: return "D";
  case 31: return "E";
}
```

#### Do

```jsx
return LETTERS[code];
```


### TypeScript

#### Don't

```ts
switch (code) {
  case 1: return "a"; case 2: return "b"; case 3: return "c"; case 4: return "d"; case 5: return "e";
  case 6: return "f"; case 7: return "g"; case 8: return "h"; case 9: return "i"; case 10: return "j";
  case 11: return "k"; case 12: return "l"; case 13: return "m"; case 14: return "n"; case 15: return "o";
  case 16: return "p"; case 17: return "q"; case 18: return "r"; case 19: return "s"; case 20: return "t";
  case 21: return "u"; case 22: return "v"; case 23: return "w"; case 24: return "x"; case 25: return "y";
  case 26: return "z"; case 27: return "A"; case 28: return "B"; case 29: return "C"; case 30: return "D";
  case 31: return "E";
}
```

#### Do

```ts
return LETTERS[code];
```


### TSX

#### Don't

```tsx
switch (code) {
  case 1: return "a"; case 2: return "b"; case 3: return "c"; case 4: return "d"; case 5: return "e";
  case 6: return "f"; case 7: return "g"; case 8: return "h"; case 9: return "i"; case 10: return "j";
  case 11: return "k"; case 12: return "l"; case 13: return "m"; case 14: return "n"; case 15: return "o";
  case 16: return "p"; case 17: return "q"; case 18: return "r"; case 19: return "s"; case 20: return "t";
  case 21: return "u"; case 22: return "v"; case 23: return "w"; case 24: return "x"; case 25: return "y";
  case 26: return "z"; case 27: return "A"; case 28: return "B"; case 29: return "C"; case 30: return "D";
  case 31: return "E";
}
```

#### Do

```tsx
return LETTERS[code];
```
