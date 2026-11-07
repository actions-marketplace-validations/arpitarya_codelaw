---
id: js/no-unnecessary-fetch-options
title: "No method: \"GET\" in fetch options"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-unnecessary-fetch-options]
since: 0.5.0
---

`fetch` is not told `method: "GET"`: it is the default. An options object that held only that is dropped.

**Why:** The option restates the default and hides the options that matter.

```codelaw
utils:
  get-method:
    kind: pair
    has: { field: key, regex: '^["'']?method["'']?$' }
    all: [{ has: { field: value, regex: '^["''`]GET["''`]$' } }]
match:
  any:
    - pattern: "fetch($U, $O)"
      all:
        - has: { field: arguments, has: { kind: object, pattern: $O, has: { matches: get-method }, not: { has: { any: [{ kind: pair, not: { matches: get-method } }, { kind: shorthand_property_identifier }, { kind: spread_element }, { kind: method_definition }] } } } }
        - pattern: $ONLY
    - matches: get-method
      all: [{ pattern: $PAIR }]
      inside:
        kind: object
        not: { has: { kind: spread_element } }
        has: { any: [{ kind: pair, not: { matches: get-method } }, { kind: shorthand_property_identifier }, { kind: method_definition }] }
        inside: { kind: arguments, inside: { kind: call_expression, has: { field: function, regex: '^fetch$' } } }
message: "`GET` is the default method"
fix:
  - replace: { target: $ONLY, with: "fetch($U)" }
  - remove: $PAIR
```

## Examples

### JavaScript

#### Don't

```js
const res = await fetch(url, { method: "GET" });
```

#### Do

```js
const res = await fetch(url);
```


### JSX

#### Don't

```jsx
const res = await fetch(url, { method: "GET" });
```

#### Do

```jsx
const res = await fetch(url);
```


### TypeScript

#### Don't

```ts
const res = await fetch(url, { method: "GET" });
```

#### Do

```ts
const res = await fetch(url);
```


### TSX

#### Don't

```tsx
const res = await fetch(url, { method: "GET" });
```

#### Do

```tsx
const res = await fetch(url);
```
