---
id: js/no-unnecessary-splice
title: "push, pop, shift or unshift rather than splice() at an end"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/no-unnecessary-splice]
since: 0.5.0
---

A `splice` call used as a statement that adds or removes one end of the array is the method made for it: `splice(0, 1)` is `shift()`, `splice(0, 0, x)` is `unshift(x)`, `splice(array.length - 1, 1)` is `pop()` and `splice(array.length, 0, x)` is `push(x)`.

**Why:** The named method says which end and which way; the splice arguments make the reader work it out.

```codelaw
utils:
  simple:
    any:
      - kind: identifier
      - kind: this
      - kind: member_expression
        not: { has: { stopBy: end, any: [{ kind: call_expression }, { kind: optional_chain }] } }
match:
  kind: expression_statement
  any:
    - all:
        - pattern: "$A.splice(0, 1);"
        - pattern: $W1
    - all:
        - pattern: "$A.splice(0, 0, $$$X);"
        - pattern: $W2
    - all:
        - pattern: "$A.splice($A.length - 1, 1);"
        - pattern: $W3
    - all:
        - pattern: "$A.splice($A.length, 0, $$$Y);"
        - pattern: $W4
constraints:
  A: { matches: simple }
message: "Use the array method for this end"
fix:
  - replace: { target: $W1, with: "$A.shift();" }
  - replace: { target: $W2, with: "$A.unshift($$$X);" }
  - replace: { target: $W3, with: "$A.pop();" }
  - replace: { target: $W4, with: "$A.push($$$Y);" }
```

## Examples

### JavaScript

#### Don't

```js
queue.splice(0, 1);
queue.splice(queue.length, 0, job);
```

#### Do

```js
queue.shift();
queue.push(job);
```


### JSX

#### Don't

```jsx
queue.splice(0, 1);
queue.splice(queue.length, 0, job);
```

#### Do

```jsx
queue.shift();
queue.push(job);
```


### TypeScript

#### Don't

```ts
queue.splice(0, 1);
queue.splice(queue.length, 0, job);
```

#### Do

```ts
queue.shift();
queue.push(job);
```


### TSX

#### Don't

```tsx
queue.splice(0, 1);
queue.splice(queue.length, 0, job);
```

#### Do

```tsx
queue.shift();
queue.push(job);
```
