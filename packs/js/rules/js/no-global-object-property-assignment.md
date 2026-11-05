---
id: js/no-global-object-property-assignment
title: "No properties on the global object"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "readers of the global become importers of a module"
stability: preview
ported_from: [eslint-plugin-unicorn/no-global-object-property-assignment]
since: 0.5.0
---

A property is not added to `window`, `globalThis`, `self` or `global` (the event handlers and `location` are left alone).

**Why:** Globals are shared by every script on the page and collide silently.

```codelaw
match:
  kind: assignment_expression
  has:
    field: left
    kind: member_expression
    has:
      field: object
      kind: identifier
      regex: ^(window|globalThis|self|global)$
    all:
    - has:
        field: property
        not:
          regex: ^(location|onerror|onload|onunhandledrejection|name|status)$
message: Writing to the global object leaks state to every script; export it instead
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
window.appConfig = load();
```

#### Do

```js
export const appConfig = load();
```

### JSX

#### Don't

```jsx
window.appConfig = load();
```

#### Do

```jsx
export const appConfig = load();
```

### TypeScript

#### Don't

```ts
window.appConfig = load();
```

#### Do

```ts
export const appConfig = load();
```

### TSX

#### Don't

```tsx
window.appConfig = load();
```

#### Do

```tsx
export const appConfig = load();
```
