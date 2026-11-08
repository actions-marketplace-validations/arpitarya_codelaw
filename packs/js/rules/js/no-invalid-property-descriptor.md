---
id: js/no-invalid-property-descriptor
title: "No property descriptor with both a value and an accessor"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the property holds a value or is computed is the author's to say"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/no-invalid-property-descriptor]
since: 0.5.0
---

A descriptor passed to `Object.defineProperty` or `Reflect.defineProperty` does not have both a data part (`value`, `writable`) and an accessor (`get`, `set`): defining it throws a `TypeError`.

**Why:** A property is either a stored value or a getter and setter, never both.

```codelaw
utils:
  data-key:
    kind: pair
    has: { field: key, regex: '^["'']?(value|writable)["'']?$' }
  accessor:
    any:
      - kind: pair
        has: { field: key, regex: '^["'']?(get|set)["'']?$' }
      - kind: method_definition
        regex: '^(async\s+)?(get|set)\s*\('
match:
  pattern: "$R.$M($O, $K, $D)"
constraints:
  R: { regex: '^(Object|Reflect)$' }
  M: { regex: '^defineProperty$' }
  D: { kind: object, all: [{ has: { matches: data-key } }, { has: { matches: accessor } }] }
message: "A descriptor has a value or accessors, not both"
```

## How to fix

1. Keep `value`/`writable` for a stored property, or `get`/`set` for a computed one, and remove the other.

## Examples

### JavaScript

#### Don't

```js
Object.defineProperty(config, "port", { value: 80, get() { return 80; } });
```

#### Do

```js
Object.defineProperty(config, "port", { get() { return 80; } });
```


### JSX

#### Don't

```jsx
Object.defineProperty(config, "port", { value: 80, get() { return 80; } });
```

#### Do

```jsx
Object.defineProperty(config, "port", { get() { return 80; } });
```


### TypeScript

#### Don't

```ts
Object.defineProperty(config, "port", { value: 80, get() { return 80; } });
```

#### Do

```ts
Object.defineProperty(config, "port", { get() { return 80; } });
```


### TSX

#### Don't

```tsx
Object.defineProperty(config, "port", { value: 80, get() { return 80; } });
```

#### Do

```tsx
Object.defineProperty(config, "port", { get() { return 80; } });
```
