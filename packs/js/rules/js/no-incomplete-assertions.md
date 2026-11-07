---
id: js/no-incomplete-assertions
title: "No chai assertion that stops at a language chain"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which assertion was meant is the author's"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/no-incomplete-assertions]
since: 0.5.0
---

A chai `expect(…)` statement ends in an assertion, not in a language chain such as `.to`, `.be`, `.have` or `.not`.

**Why:** `expect(x).to.be;` asserts nothing and always passes.

```codelaw
match:
  kind: expression_statement
  regex: '^expect\((?s:.*)\)(\.(to|be|been|is|that|which|and|has|have|with|at|of|same|but|does|still|also|not|deep|nested|own|ordered|any|all))+;?$'
message: "This assertion asserts nothing"
```

## How to fix

1. Finish the chain with the assertion that was meant: `.true`, `.equal(…)`, `.empty`.

## Examples

### JavaScript

#### Don't

```js
expect(result).to.be;
```

#### Do

```js
expect(result).to.be.true;
```


### JSX

#### Don't

```jsx
expect(result).to.be;
```

#### Do

```jsx
expect(result).to.be.true;
```


### TypeScript

#### Don't

```ts
expect(result).to.be;
```

#### Do

```ts
expect(result).to.be.true;
```


### TSX

#### Don't

```tsx
expect(result).to.be;
```

#### Do

```tsx
expect(result).to.be.true;
```
