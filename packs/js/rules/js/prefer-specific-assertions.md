---
id: js/prefer-specific-assertions
title: "toBeNull(), toBeUndefined() and toHaveLength() rather than toBe() with them"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the specific matcher exists in Jest and Vitest; Jasmine spells some of them differently"
stability: preview
ported_from: [eslint-plugin-sonarjs/prefer-specific-assertions]
since: 0.5.0
---

`expect(x).toBe(null)` is `expect(x).toBeNull()`, `expect(x).toBe(undefined)` is `expect(x).toBeUndefined()`, and `expect(list.length).toBe(n)` is `expect(list).toHaveLength(n)`. Upstream also rewrites `toBe(true)` on comparisons and Node's `assert`; those are not reported here.

**Why:** The specific matcher's failure message shows the value, not just `expected null`.

```codelaw
match:
  kind: call_expression
  any:
    - all:
        - pattern: "expect($A).toBe(null)"
        - pattern: $W1
    - all:
        - pattern: "expect($A).toBe(undefined)"
        - pattern: $W2
    - all:
        - pattern: "expect($L.length).toBe($N)"
        - pattern: $W3
message: "Use the specific matcher"
fix:
  - replace: { target: $W1, with: "expect($A).toBeNull()" }
  - replace: { target: $W2, with: "expect($A).toBeUndefined()" }
  - replace: { target: $W3, with: "expect($L).toHaveLength($N)" }
```

## Examples

### JavaScript

#### Don't

```js
expect(error).toBe(null);
```

#### Do

```js
expect(error).toBeNull();
```


### JSX

#### Don't

```jsx
expect(error).toBe(null);
```

#### Do

```jsx
expect(error).toBeNull();
```


### TypeScript

#### Don't

```ts
expect(error).toBe(null);
```

#### Do

```ts
expect(error).toBeNull();
```


### TSX

#### Don't

```tsx
expect(error).toBe(null);
```

#### Do

```tsx
expect(error).toBeNull();
```
