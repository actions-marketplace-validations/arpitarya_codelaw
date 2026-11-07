---
id: js/no-undefined-assignment
title: "No assignment of undefined"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "`null` and `undefined` read differently to `??`, defaults and JSON; which one is meant is the author's"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/no-undefined-assignment]
since: 0.5.0
---

A variable is not assigned `undefined`; an absent value is `null`, and `undefined` stays what the language gives a variable that was never set.

**Why:** Keeping `undefined` for never-set lets a reader tell a forgotten value from one cleared on purpose.

```codelaw
match:
  pattern: "$X = undefined"
  kind: assignment_expression
message: "Assign `null` for an absent value"
```

## How to fix

1. Assign `null`, and check the code that reads it treats `null` as absent.

## Examples

### JavaScript

#### Don't

```js
session = undefined;
```

#### Do

```js
session = null;
```


### JSX

#### Don't

```jsx
session = undefined;
```

#### Do

```jsx
session = null;
```


### TypeScript

#### Don't

```ts
session = undefined;
```

#### Do

```ts
session = null;
```


### TSX

#### Don't

```tsx
session = undefined;
```

#### Do

```tsx
session = null;
```
