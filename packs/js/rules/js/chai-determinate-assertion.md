---
id: js/chai-determinate-assertion
title: "No negated Chai throw assertion with an argument"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the code should throw nothing, or throw something else, is the author's to state"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-sonarjs/chai-determinate-assertion]
since: 0.5.0
---

`expect(fn).to.not.throw(ReferenceError)` passes when `fn` throws nothing and when it throws a `TypeError`: it has two reasons to succeed. Assert the one that is meant — `to.not.throw()` or `to.throw(TypeError)`. Upstream also reports other undetermined Chai forms (`not.include` with a key, `not.have.property` with a value, …); those are not reported here.

**Why:** A test that passes for more than one reason does not tell you which happened.

```codelaw
match:
  kind: call_expression
  all:
    - has:
        field: function
        kind: member_expression
        regex: '^expect\([\s\S]*\)(\.to)?\.not\.throws?$'
    - has: { field: arguments, has: { nthChild: 1 } }
message: "This assertion can pass for more than one reason"
```

## How to fix

1. If nothing should be thrown, drop the argument: `.to.not.throw()`.
2. If a different error should be thrown, assert it: `.to.throw(TypeError)`.

## Examples

### JavaScript

#### Don't

```js
expect(run).to.not.throw(ReferenceError);
```

#### Do

```js
expect(run).to.not.throw();
```


### JSX

#### Don't

```jsx
expect(run).to.not.throw(ReferenceError);
```

#### Do

```jsx
expect(run).to.not.throw();
```


### TypeScript

#### Don't

```ts
expect(run).to.not.throw(ReferenceError);
```

#### Do

```ts
expect(run).to.not.throw();
```


### TSX

#### Don't

```tsx
expect(run).to.not.throw(ReferenceError);
```

#### Do

```tsx
expect(run).to.not.throw();
```
