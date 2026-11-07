---
id: js/max-nested-calls
title: "No call nested more than three deep in another call's arguments"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the intermediate values need names, which are the author's"
fix_exception: value
options: { max: { type: "integer", default: 3, description: "the deepest a call may sit in the arguments of others" } }
stability: preview
ported_from: [eslint-plugin-unicorn/max-nested-calls]
since: 0.5.0
---

A call or `new` passed as an argument to a call, passed to a call, … is reported once it sits more than `max` (3) deep. Method chains (`a().b().c()`) are not nesting, and a function, class or JSX element starts the count again.

**Why:** Past three levels the reader has to unwind the expression from the inside out.

```codelaw
native: js/max-nested-calls
```

## How to fix

1. Assign the innermost calls to `const`s with names, and pass the names in.

## Examples

### JavaScript

#### Don't

```js
foo(bar(baz(qux())));
```

#### Do

```js
const value = baz(qux());
foo(bar(value));
```


### JSX

#### Don't

```jsx
foo(bar(baz(qux())));
```

#### Do

```jsx
const value = baz(qux());
foo(bar(value));
```


### TypeScript

#### Don't

```ts
foo(bar(baz(qux())));
```

#### Do

```ts
const value = baz(qux());
foo(bar(value));
```


### TSX

#### Don't

```tsx
foo(bar(baz(qux())));
```

#### Do

```tsx
const value = baz(qux());
foo(bar(value));
```
