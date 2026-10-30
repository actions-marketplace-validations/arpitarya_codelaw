---
id: react/rule-suppression
title: "The hooks rules are not switched off"
languages: [javascript, jsx, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-react-hooks/rule-suppression]
since: 0.5.0
---

No `eslint-disable` comment switches off a React hooks rule (`react-hooks/exhaustive-deps`, `react-hooks/rules-of-hooks`, …). The fix deletes the comment, which changes no code; the findings it hid come back.

**Why:** A suppressed hooks rule usually hides a real stale-closure or ordering bug, and the React Compiler skips the component.

```codelaw
match:
  kind: comment
  regex: 'eslint-disable(-next-line|-line)?\s[^\n]*react-hooks/'
message: "Don't switch off the React hooks rules; fix what they report"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
function T({ id }) {
  useEffect(() => {
    load(id);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
  return null;
}
```

#### Do

```js
function T({ id }) {
  useEffect(() => {
    load(id);
  }, [id]);
  return null;
}
```

#### Fixed

```js
function T({ id }) {
  useEffect(() => {
    load(id);
  }, []);
  return null;
}
```

### JSX

#### Don't

```jsx
function T({ id }) {
  useEffect(() => {
    load(id);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
  return null;
}
```

#### Do

```jsx
function T({ id }) {
  useEffect(() => {
    load(id);
  }, [id]);
  return null;
}
```

#### Fixed

```jsx
function T({ id }) {
  useEffect(() => {
    load(id);
  }, []);
  return null;
}
```

### TSX

#### Don't

```tsx
function T({ id }) {
  useEffect(() => {
    load(id);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
  return null;
}
```

#### Do

```tsx
function T({ id }) {
  useEffect(() => {
    load(id);
  }, [id]);
  return null;
}
```

#### Fixed

```tsx
function T({ id }) {
  useEffect(() => {
    load(id);
  }, []);
  return null;
}
```
