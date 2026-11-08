---
id: js/constructor-for-side-effects
title: "No object constructed only for its side effects"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "keeping the object or turning the effect into a function is the author's choice"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/constructor-for-side-effects]
since: 0.5.0
---

A `new` expression is not a statement on its own: the object it builds is kept, or the work is a function call.

**Why:** Constructing an object and dropping it means the constructor does work a function should do, and readers look for the object.

```codelaw
match:
  kind: expression_statement
  has: { kind: new_expression }
message: "The object built here is thrown away"
```

## How to fix

1. Assign the object to a name and use it, or call a function that does the work.

## Examples

### JavaScript

#### Don't

```js
new Logger(config);
```

#### Do

```js
const logger = new Logger(config);
logger.start();
```


### JSX

#### Don't

```jsx
new Logger(config);
```

#### Do

```jsx
const logger = new Logger(config);
logger.start();
```


### TypeScript

#### Don't

```ts
new Logger(config);
```

#### Do

```ts
const logger = new Logger(config);
logger.start();
```


### TSX

#### Don't

```tsx
new Logger(config);
```

#### Do

```tsx
const logger = new Logger(config);
logger.start();
```
