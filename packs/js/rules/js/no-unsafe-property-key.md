---
id: js/no-unsafe-property-key
title: "No object, array or BigInt literal as a property key"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the key should be a string, a number or a `Map` entry is the author's call"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/no-unsafe-property-key]
since: 0.5.0
---

A property key is a string or a symbol; anything else is converted first. An object literal becomes `"[object Object]"`, an array its joined elements, a BigInt its digits — so `obj[{}]` and `obj[[]]` collide with every other such key. A literal object, array or BigInt used as a computed key, in a member access or an object literal, is reported. Upstream also follows variables and types to such values; that needs data-flow, and is not reported here.

**Why:** Two different objects used as keys silently overwrite each other.

```codelaw
utils:
  bad:
    any:
      - kind: object
      - kind: array
      - kind: number
        regex: 'n$'
match:
  any:
    - all:
        - kind: subscript_expression
        - has: { field: index, matches: bad }
    - all:
        - kind: computed_property_name
        - has: { matches: bad }
message: "This key is converted to a string; use a string key or a `Map`"
```

## How to fix

1. If the text form is what you mean, write it: `String(key)`, `key.join(',')`.
2. If the object itself is the key, use a `Map` or `WeakMap`.

## Examples

### JavaScript

#### Don't

```js
cache[[a, b]] = value;
```

#### Do

```js
cache[`${a},${b}`] = value;
```


### JSX

#### Don't

```jsx
cache[[a, b]] = value;
```

#### Do

```jsx
cache[`${a},${b}`] = value;
```


### TypeScript

#### Don't

```ts
cache[[a, b]] = value;
```

#### Do

```ts
cache[`${a},${b}`] = value;
```


### TSX

#### Don't

```tsx
cache[[a, b]] = value;
```

#### Do

```tsx
cache[`${a},${b}`] = value;
```
