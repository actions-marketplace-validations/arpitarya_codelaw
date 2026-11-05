---
id: react/capitalized-calls
title: "Components are rendered with JSX, not called"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "restructure: a component call becomes a JSX element, and a capitalised helper that is not a component is renamed"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-react-hooks/capitalized-calls]
since: 0.5.0
---

Inside a component or hook, a capitalised function — a component, by React's naming — is not called directly (`Header()`); it is rendered (`<Header />`). The JavaScript globals (`Number`, `String`, `Date`, …) are not reported.

**Why:** Calling a component runs its hooks inside the caller's, which breaks the rules of hooks the moment the call is conditional, and skips its own memoisation.

```codelaw
utils:
  comp-or-hook:
    any:
      - { kind: function_declaration, has: { field: name, regex: '^([A-Z]|use[A-Z])' } }
      - any: [{ kind: arrow_function }, { kind: function_expression }]
        inside: { kind: variable_declarator, has: { field: name, regex: '^([A-Z]|use[A-Z])' } }
match:
  kind: call_expression
  has: { field: function, kind: identifier, regex: '^[A-Z]', not: { regex: '^(Array|ArrayBuffer|BigInt|Boolean|DataView|Date|Error|EvalError|Function|Intl|JSON|Map|Math|Number|Object|Promise|Proxy|RangeError|ReferenceError|Reflect|RegExp|Set|String|Symbol|SyntaxError|TypeError|URIError|URL|URLSearchParams|WeakMap|WeakRef|WeakSet|Uint8Array|Int8Array|Uint16Array|Int16Array|Uint32Array|Int32Array|Float32Array|Float64Array|Headers|Request|Response|FormData|Blob|File|AbortController|TextEncoder|TextDecoder)$' }, pattern: $F }
  inside: { stopBy: end, matches: comp-or-hook }
message: "Render `$F` as `<$F />` instead of calling it"
```

## How to fix

1. Render the component as JSX: `<Header />`.
2. If it is a plain helper, give it a lower-case name.

## Examples

### JavaScript

#### Don't

```js
function Page() {
  return <main>{Header()}</main>;
}
```

#### Do

```js
function Page() {
  return <main><Header /></main>;
}
```

### JSX

#### Don't

```jsx
function Page() {
  return <main>{Header()}</main>;
}
```

#### Do

```jsx
function Page() {
  return <main><Header /></main>;
}
```

### TSX

#### Don't

```tsx
function Page() {
  return <main>{Header()}</main>;
}
```

#### Do

```tsx
function Page() {
  return <main><Header /></main>;
}
```
