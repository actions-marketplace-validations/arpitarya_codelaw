---
id: js/no-unreadable-new-expression
title: "No member access straight off a new expression"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "splitting the expression needs a name for the instance, which is the author's"
fix_exception: value
stability: preview
ported_from: [eslint-plugin-unicorn/no-unreadable-new-expression]
since: 0.5.0
---

`new Foo().bar` reads `bar` off a new instance, while `new Foo.Bar()` constructs `Foo.Bar` — two shapes that look alike and do opposite things. Member access or a call directly on a `new` expression is reported; name the instance first. Upstream also reports complex constructor expressions; those parse differently, and are not reported here.

**Why:** `new`'s precedence depends on whether there's an argument list, and few readers keep that in their head.

```codelaw
match:
  kind: member_expression
  has: { field: object, kind: new_expression }
message: "Name the new instance before reading from it"
```

## How to fix

1. Assign the instance to a `const` with a name, then read the member from it.

## Examples

### JavaScript

#### Don't

```js
const bar = new Foo().getBar();
```

#### Do

```js
const foo = new Foo();
const bar = foo.getBar();
```


### JSX

#### Don't

```jsx
const bar = new Foo().getBar();
```

#### Do

```jsx
const foo = new Foo();
const bar = foo.getBar();
```


### TypeScript

#### Don't

```ts
const bar = new Foo().getBar();
```

#### Do

```ts
const foo = new Foo();
const bar = foo.getBar();
```


### TSX

#### Don't

```tsx
const bar = new Foo().getBar();
```

#### Do

```tsx
const foo = new Foo();
const bar = foo.getBar();
```
