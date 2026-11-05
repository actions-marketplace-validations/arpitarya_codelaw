---
id: sec-js/no-prototype-pollution
title: "No request value used as a key to write or merge"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "defining an own property cannot reach `Object.prototype` even for the key `__proto__`, and the reviewer confirms a plain assignment (setters, frozen objects) is not relied on"
stability: preview
ported_from: [eslint-plugin-security/detect-object-injection]
security: { cwe: [1321], owasp: ["A08:2025"], severity: high }
since: 0.3.0
---

An object is not written by a key that comes from the request (`target[key] = value`), and `merge`, `defaultsDeep` and `$.extend(true, …)` are not given a request object.

**Why:** A key of `__proto__` or `constructor` writes onto `Object.prototype`, so every object in the process gains the attacker's property: privilege flags, crashes, even code paths (CWE-1321).

```codelaw
match:
  any:
    - all:
        - pattern: $O[$K] = $V
        - pattern: $ASSIGN
    - pattern: _.merge($T, $K)
    - pattern: _.merge($T, $K, $$$R)
    - pattern: _.merge($A1, $T, $K)
    - pattern: lodash.merge($T, $K)
    - pattern: lodash.merge($T, $K, $$$R)
    - pattern: lodash.merge($A1, $T, $K)
    - pattern: merge($T, $K)
    - pattern: merge($T, $K, $$$R)
    - pattern: merge($A1, $T, $K)
    - pattern: deepmerge($T, $K)
    - pattern: deepmerge($T, $K, $$$R)
    - pattern: deepmerge($A1, $T, $K)
    - pattern: _.defaultsDeep($T, $K)
    - pattern: _.defaultsDeep($T, $K, $$$R)
    - pattern: _.defaultsDeep($A1, $T, $K)
    - pattern: defaultsDeep($T, $K)
    - pattern: defaultsDeep($T, $K, $$$R)
    - pattern: defaultsDeep($A1, $T, $K)
    - pattern: _.mergeWith($T, $K)
    - pattern: _.mergeWith($T, $K, $$$R)
    - pattern: _.mergeWith($A1, $T, $K)
    - pattern: mergeWith($T, $K)
    - pattern: mergeWith($T, $K, $$$R)
    - pattern: mergeWith($A1, $T, $K)
    - pattern: $.extend($T, $K)
    - pattern: $.extend($T, $K, $$$R)
    - pattern: $.extend($A1, $T, $K)
    - pattern: extend($T, $K)
    - pattern: extend($T, $K, $$$R)
    - pattern: extend($A1, $T, $K)
    - pattern: _.set($T, $K)
    - pattern: _.set($T, $K, $$$R)
    - pattern: _.set($A1, $T, $K)
    - pattern: lodash.set($T, $K)
    - pattern: lodash.set($T, $K, $$$R)
    - pattern: lodash.set($A1, $T, $K)
    - pattern: set($T, $K)
    - pattern: set($T, $K, $$$R)
    - pattern: set($A1, $T, $K)
where:
  $K: { from_source: { tables: [sources/express, sources/koa, sources/nextjs] } }
message: "`$K` comes from the request and is used as a key or merged into an object; a key of `__proto__` pollutes every object"
fix:
  - replace: {target: $ASSIGN, with: 'Object.defineProperty($O, $K, { value: $V, writable: true, enumerable: true, configurable: true })'}
```

## How to fix

1. Use a `Map` for keys a client chooses (`settings.set(key, value)`), or create the object with `Object.create(null)`.
2. Reject `__proto__`, `constructor` and `prototype` as keys, or check `Object.hasOwn(allowed, key)` against a list.
3. Use a merge that ignores those keys (lodash 4.17.21 or later, `deepmerge` with `isMergeableObject`), or `structuredClone`, instead of copying request data into a live object.

## Examples

### JavaScript

#### Don't

```js
const { key, value } = req.body;
config[key] = value;
```

#### Fixed

```js
const { key, value } = req.body;
Object.defineProperty(config, key, { value: value, writable: true, enumerable: true, configurable: true });
```

#### Do

```js
const { key, value } = req.body;
settings.set(key, value);
```

### JSX

#### Don't

```jsx
const { key, value } = req.body;
config[key] = value;
```

#### Fixed

```jsx
const { key, value } = req.body;
Object.defineProperty(config, key, { value: value, writable: true, enumerable: true, configurable: true });
```

#### Do

```jsx
const { key, value } = req.body;
settings.set(key, value);
```

### TypeScript

#### Don't

```ts
const { key, value } = req.body;
config[key] = value;
```

#### Fixed

```ts
const { key, value } = req.body;
Object.defineProperty(config, key, { value: value, writable: true, enumerable: true, configurable: true });
```

#### Do

```ts
const { key, value } = req.body;
settings.set(key, value);
```

### TSX

#### Don't

```tsx
const { key, value } = req.body;
config[key] = value;
```

#### Fixed

```tsx
const { key, value } = req.body;
Object.defineProperty(config, key, { value: value, writable: true, enumerable: true, configurable: true });
```

#### Do

```tsx
const { key, value } = req.body;
settings.set(key, value);
```
