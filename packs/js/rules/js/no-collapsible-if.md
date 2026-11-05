---
id: js/no-collapsible-if
title: "Merge nested ifs"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-sonarjs/no-collapsible-if]
since: 0.5.0
---

An `if` without `else` whose only statement is another `if` without `else` is merged into one.

**Why:** One condition reads faster than two levels of nesting.

```codelaw
match:
  kind: if_statement
  not:
    has:
      field: alternative
      regex: .
  has:
    field: condition
    kind: parenthesized_expression
    has:
      pattern: $A
  all:
  - has:
      field: consequence
      kind: statement_block
      has:
        kind: if_statement
        not:
          has:
            field: alternative
            regex: .
        has:
          field: condition
          has:
            pattern: $B
        all:
        - has:
            field: consequence
            pattern: $BODY
      not:
        has:
          nthChild: 2
message: This `if` holds only another `if`; join the conditions with `&&`
fix: if (($A) && ($B)) $BODY
```

## Examples

### JavaScript

#### Don't

```js
if (user) {
  if (user.admin) {
    grant();
  }
}
```

#### Do

```js
if ((user) && (user.admin)) {
    grant();
  }
```

### JSX

#### Don't

```jsx
if (user) {
  if (user.admin) {
    grant();
  }
}
```

#### Do

```jsx
if ((user) && (user.admin)) {
    grant();
  }
```

### TypeScript

#### Don't

```ts
if (user) {
  if (user.admin) {
    grant();
  }
}
```

#### Do

```ts
if ((user) && (user.admin)) {
    grant();
  }
```

### TSX

#### Don't

```tsx
if (user) {
  if (user.admin) {
    grant();
  }
}
```

#### Do

```tsx
if ((user) && (user.admin)) {
    grant();
  }
```
