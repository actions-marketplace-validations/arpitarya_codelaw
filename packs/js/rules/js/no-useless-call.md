---
id: js/no-useless-call
title: "No needless call or apply"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "a call on a plain identifier with `undefined`, or on `o.m` with `o` as the receiver, is the plain call exactly; a `null` this or a complex callee stays a suggestion"
stability: preview
ported_from: [eslint/no-useless-call]
since: 0.5.0
---

`.call()` and `.apply()` are not used where a plain call does the same.

**Why:** `f.call(undefined, a)` is `f(a)` and `obj.m.call(obj, a)` is `obj.m(a)`; the long form hides that.

```codelaw
match:
  any:
    - pattern: $F.call(undefined, $$$A)
      all: [{ pattern: $CU }]
    - pattern: $F.call(null, $$$A)
      all: [{ pattern: $CN }]
    - pattern: $O.$M.call($O, $$$A)
      all: [{ pattern: $CM }]
message: "This `.call()` is not needed; call the function directly"
auto_when:
  regex: '^([A-Za-z_$][\w$]*\.call\(\s*undefined\b|[A-Za-z_$][\w$]*\.[A-Za-z_$][\w$]*\.call\()'
fix:
  - replace: { target: $CU, with: "$F($$$A)" }
  - replace: { target: $CN, with: "$F($$$A)" }
  - replace: { target: $CM, with: "$O.$M($$$A)" }
```

## How to fix

1. Call the function directly. The fix does it for a plain identifier called with `undefined`, and for `o.m.call(o, ...)`; any other callee is offered for a reviewer to confirm.

## Examples

### JavaScript

#### Don't

```js
greet.call(undefined, 'Ada');
```

#### Fixed

```js
greet('Ada');
```

#### Do

```js
greet('Ada');
```

### JSX

#### Don't

```jsx
greet.call(undefined, 'Ada');
```

#### Fixed

```jsx
greet('Ada');
```

#### Do

```jsx
greet('Ada');
```

### TypeScript

#### Don't

```ts
greet.call(undefined, 'Ada');
```

#### Fixed

```ts
greet('Ada');
```

#### Do

```ts
greet('Ada');
```

### TSX

#### Don't

```tsx
greet.call(undefined, 'Ada');
```

#### Fixed

```tsx
greet('Ada');
```

#### Do

```tsx
greet('Ada');
```
