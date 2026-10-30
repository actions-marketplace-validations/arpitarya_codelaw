---
id: react/jsx-pascal-case
title: "Component names are PascalCase"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "value: the new name is the author's, and every import of the component follows it"
stability: preview
ported_from: [eslint-plugin-react/jsx-pascal-case]
since: 0.5.0
---

A component named in JSX is PascalCase: no underscores (`<User_Card>`) and not all capitals (`<USERCARD>`).

**Why:** JSX reads a capitalised name as a component; one casing for all of them keeps imports and file names predictable.

```codelaw
match:
  any: [{ kind: jsx_opening_element }, { kind: jsx_self_closing_element }]
  has: { field: name, kind: identifier, regex: '^[A-Z]([A-Za-z0-9]*_[A-Za-z0-9_]*|[A-Z0-9]{2,})$', pattern: $N }
message: "`$N` isn't PascalCase"
```

## How to fix

1. Rename the component to PascalCase (`UserCard`) where it's defined and everywhere it's imported.

## Examples

### JavaScript

#### Don't

```js
const c = <User_Card />;
```

#### Do

```js
const c = <UserCard />;
```

### JSX

#### Don't

```jsx
const c = <User_Card />;
```

#### Do

```jsx
const c = <UserCard />;
```

### TSX

#### Don't

```tsx
const c = <User_Card />;
```

#### Do

```tsx
const c = <UserCard />;
```
