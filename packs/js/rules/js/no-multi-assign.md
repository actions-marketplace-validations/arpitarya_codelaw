---
id: js/no-multi-assign
title: "No chained assignment"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "a chain of identifier targets written as a whole expression statement splits right to left with the same evaluation order; member targets, declarators and chains inside larger expressions carry no edit"
stability: preview
ported_from: [eslint/no-multi-assign]
since: 0.5.0
---

An assignment's value is not itself an assignment (`a = b = c`).

**Why:** Chained assignments read left to right but run right to left, and with `const a = b = 1` create an implicit global.

```codelaw
match:
  any:
    - kind: assignment_expression
      inside: { kind: expression_statement, stopBy: neighbor }
      all:
        - has: { field: left, kind: identifier, pattern: $A }
        - has:
            field: right
            kind: assignment_expression
            all:
              - has: { field: left, kind: identifier, pattern: $B }
              - has: { field: right, pattern: $E }
    - kind: assignment_expression
      has: { field: right, kind: assignment_expression }
    - kind: variable_declarator
      has: { field: value, kind: assignment_expression }
message: "Split this chained assignment into separate statements"
fix: "$B = $E;\n$A = $B"
```

## How to fix

1. Write one assignment per statement.

## Examples

### JavaScript

#### Don't

```js
a = b = 0;
```

#### Fixed

```js
b = 0;
a = b;
```

#### Do

```js
a = 0;
b = 0;
```

### JSX

#### Don't

```jsx
a = b = 0;
```

#### Fixed

```jsx
b = 0;
a = b;
```

#### Do

```jsx
a = 0;
b = 0;
```

### TypeScript

#### Don't

```ts
a = b = 0;
```

#### Fixed

```ts
b = 0;
a = b;
```

#### Do

```ts
a = 0;
b = 0;
```

### TSX

#### Don't

```tsx
a = b = 0;
```

#### Fixed

```tsx
b = 0;
a = b;
```

#### Do

```tsx
a = 0;
b = 0;
```
