---
id: js/no-hook-setter-in-body
title: "No state setter called in a component's body"
languages: [jsx, tsx]
fix: manual
fix_reason: "where the update belongs — an effect, a handler, the initial state — is the author's"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/no-hook-setter-in-body]
since: 0.5.0
---

A `useState` setter is not called as a statement of the component's own body: each render sets state, which renders again, without end.

**Why:** Setting state while rendering loops until React stops it with an error.

```codelaw
match:
  kind: expression_statement
  pattern: "$SET($$$A);"
  inside:
    kind: statement_block
    has:
      any:
        - pattern: "const [$V, $SET] = useState($$$I);"
        - pattern: "const [$V, $SET] = React.useState($$$I);"
message: "Calling `$SET` while rendering renders again, forever"
```

## How to fix

1. Set the value as the initial state, or move the call into an event handler or a `useEffect`.

## Examples

### JSX

#### Don't

```jsx
function Clock() {
  const [now, setNow] = useState(0);
  setNow(Date.now());
  return <time>{now}</time>;
}
```

#### Do

```jsx
function Clock() {
  const [now, setNow] = useState(() => Date.now());
  return <time>{now}</time>;
}
```


### TSX

#### Don't

```tsx
function Clock() {
  const [now, setNow] = useState(0);
  setNow(Date.now());
  return <time>{now}</time>;
}
```

#### Do

```tsx
function Clock() {
  const [now, setNow] = useState(() => Date.now());
  return <time>{now}</time>;
}
```
