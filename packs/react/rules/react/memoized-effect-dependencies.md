---
id: react/memoized-effect-dependencies
title: "Effect dependencies are stable values"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: the value is wrapped in `useMemo` or `useCallback`, or moved inside the effect, with its own dependencies"
stability: preview
ported_from: [eslint-plugin-react-hooks/memoized-effect-dependencies]
since: 0.5.0
---

A value listed in an effect's dependency array is not an object, array or function created fresh in the component body.

**Why:** A value made on every render is a new dependency on every render, so the effect runs every time.

```codelaw
utils:
  comp-or-hook:
    any:
      - { kind: function_declaration, has: { field: name, regex: '^([A-Z]|use[A-Z])' } }
      - any: [{ kind: arrow_function }, { kind: function_expression }]
        inside: { kind: variable_declarator, has: { field: name, regex: '^([A-Z]|use[A-Z])' } }
match:
  kind: identifier
  pattern: $D
  inside:
    kind: array
    nthChild: 2
    inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, regex: '^(React\.)?(useEffect|useLayoutEffect|useInsertionEffect)$' } } }
  all:
    - inside:
        stopBy: end
        matches: comp-or-hook
        has:
          field: body
          has:
            kind: lexical_declaration
            has:
              kind: variable_declarator
              all:
                - has: { field: name, pattern: $D }
                - has: { field: value, any: [{ kind: object }, { kind: array }, { kind: arrow_function }, { kind: function_expression }, { kind: new_expression }] }
message: "`$D` is a new value on every render, so this effect always runs"
```

## How to fix

1. Move `$D` inside the effect, or wrap it in `useMemo` / `useCallback` with its dependencies.

## Examples

### JavaScript

#### Don't

```js
function Chat({ room }) {
  const options = { room };
  useEffect(() => connect(options), [options]);
  return null;
}
```

#### Do

```js
function Chat({ room }) {
  useEffect(() => connect({ room }), [room]);
  return null;
}
```

### JSX

#### Don't

```jsx
function Chat({ room }) {
  const options = { room };
  useEffect(() => connect(options), [options]);
  return null;
}
```

#### Do

```jsx
function Chat({ room }) {
  useEffect(() => connect({ room }), [room]);
  return null;
}
```

### TSX

#### Don't

```tsx
function Chat({ room }) {
  const options = { room };
  useEffect(() => connect(options), [options]);
  return null;
}
```

#### Do

```tsx
function Chat({ room }) {
  useEffect(() => connect({ room }), [room]);
  return null;
}
```
