---
id: js/no-built-in-override
title: "No declaration named like a built-in object"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the new name is the author's"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/no-built-in-override]
since: 0.5.0
---

A file does not declare a variable, function, class or parameter named like one of the language's built-in globals (`Object`, `Array`, `JSON`, `Promise`, `undefined`, …).

**Why:** The declaration hides the built-in for the rest of the scope, and code that expects the real one breaks.

```codelaw
match:
  kind: identifier
where:
  $$MATCH: [declaration, { shadowed: { builtins: true } }]
message: "`$$MATCH` hides the built-in of that name"
```

## How to fix

1. Rename the declaration and its uses.

## Examples

### JavaScript

#### Don't

```js
const JSON = loadConfig();
```

#### Do

```js
const config = loadConfig();
```


### JSX

#### Don't

```jsx
const JSON = loadConfig();
```

#### Do

```jsx
const config = loadConfig();
```


### TypeScript

#### Don't

```ts
const JSON = loadConfig();
```

#### Do

```ts
const config = loadConfig();
```


### TSX

#### Don't

```tsx
const JSON = loadConfig();
```

#### Do

```tsx
const config = loadConfig();
```
