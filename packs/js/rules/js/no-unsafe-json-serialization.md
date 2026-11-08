---
id: js/no-unsafe-json-serialization
title: "No Map, Set or BigInt literal handed straight to JSON.stringify"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the converted value has a different JSON shape (an array of entries, an object, a string); code reading the JSON needs review"
stability: preview
ported_from: [eslint-plugin-unicorn/no-unsafe-json-serialization]
since: 0.5.0
---

`JSON.stringify` writes a `Map` or `Set` as `{}`, and throws on a BigInt. A `new Map(…)`, `new Set(…)` or BigInt literal written in the argument — at the top or inside object and array literals — is converted first: `Object.fromEntries(map)`, `Array.from(set)`, `String(n)`. `Response.json` serializes the same way and is checked too. Upstream also reports regular expressions, functions and `NaN`, and follows `const` bindings; those have no one right conversion, or need data-flow, and are not reported here.

**Why:** The data silently disappears from the output, or the call throws.

```codelaw
utils:
  container:
    any:
      - kind: object
      - kind: array
      - kind: pair
  sink:
    kind: arguments
    inside:
      kind: call_expression
      any:
        - pattern: "JSON.stringify($$$)"
        - pattern: "Response.json($$$)"
  in_sink:
    any:
      - inside: { matches: sink }
      - inside:
          stopBy: { not: { matches: container } }
          matches: sink
match:
  matches: in_sink
  any:
    - all:
        - kind: new_expression
        - has: { field: constructor, regex: '^Set$' }
        - pattern: $SET
    - all:
        - kind: new_expression
        - has: { field: constructor, regex: '^Map$' }
        - pattern: $MAP
    - all:
        - kind: number
        - regex: 'n$'
        - pattern: $BIG
message: "JSON drops or rejects this value; convert it first"
fix:
  - replace: { target: $SET, with: "Array.from($SET)" }
  - replace: { target: $MAP, with: "Object.fromEntries($MAP)" }
  - replace: { target: $BIG, with: "String($BIG)" }
```

## Examples

### JavaScript

#### Don't

```js
JSON.stringify({ roles: new Set(["read"]) });
```

#### Do

```js
JSON.stringify({ roles: Array.from(new Set(["read"])) });
```


### JSX

#### Don't

```jsx
JSON.stringify({ roles: new Set(["read"]) });
```

#### Do

```jsx
JSON.stringify({ roles: Array.from(new Set(["read"])) });
```


### TypeScript

#### Don't

```ts
JSON.stringify({ roles: new Set(["read"]) });
```

#### Do

```ts
JSON.stringify({ roles: Array.from(new Set(["read"])) });
```


### TSX

#### Don't

```tsx
JSON.stringify({ roles: new Set(["read"]) });
```

#### Do

```tsx
JSON.stringify({ roles: Array.from(new Set(["read"])) });
```
