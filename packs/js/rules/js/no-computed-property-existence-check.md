---
id: js/no-computed-property-existence-check
title: "Object.hasOwn() rather than a computed read or `in` as an existence test"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "`Object.hasOwn` ignores inherited properties and falsy values, which is the point but changes the result when either was meant; the edit is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/no-computed-property-existence-check]
since: 0.5.0
---

Testing whether an object has a key held in a variable — `if (object[key])` or `key in object` — also finds inherited properties such as `constructor` and `toString`, and the read form misses keys whose value is falsy. `Object.hasOwn(object, key)` asks the question directly. A literal key (`object['a']`, `'a' in object`) is left, and so is a read used as a value.

**Why:** A key from input that happens to be `constructor` or `__proto__` passes the inherited check.

```codelaw
utils:
  simple:
    any:
      - kind: identifier
      - kind: this
      - kind: member_expression
        not: { has: { stopBy: end, any: [{ kind: call_expression }, { kind: optional_chain }] } }
  key:
    any:
      - kind: identifier
      - kind: member_expression
        not: { has: { stopBy: end, any: [{ kind: call_expression }, { kind: optional_chain }] } }
match:
  any:
    - all:
        - kind: subscript_expression
        - pattern: "$O[$K]"
        - any:
            - inside:
                kind: parenthesized_expression
                inside: { any: [{ kind: if_statement }, { kind: while_statement }], field: condition }
            - inside: { kind: unary_expression, regex: '^!' }
            - inside: { kind: ternary_expression, field: condition }
        - not: { has: { kind: optional_chain } }
    - all:
        - kind: binary_expression
        - pattern: "$K in $O"
constraints:
  O: { matches: simple }
  K: { matches: key }
message: "Test with `Object.hasOwn($O, $K)`; this also finds inherited properties"
fix: "Object.hasOwn($O, $K)"
```

## Examples

### JavaScript

#### Don't

```js
if (handlers[name]) {
  run(name);
}
```

#### Do

```js
if (Object.hasOwn(handlers, name)) {
  run(name);
}
```


### JSX

#### Don't

```jsx
if (handlers[name]) {
  run(name);
}
```

#### Do

```jsx
if (Object.hasOwn(handlers, name)) {
  run(name);
}
```


### TypeScript

#### Don't

```ts
if (handlers[name]) {
  run(name);
}
```

#### Do

```ts
if (Object.hasOwn(handlers, name)) {
  run(name);
}
```


### TSX

#### Don't

```tsx
if (handlers[name]) {
  run(name);
}
```

#### Do

```tsx
if (Object.hasOwn(handlers, name)) {
  run(name);
}
```
