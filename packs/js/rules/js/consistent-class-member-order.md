---
id: js/consistent-class-member-order
title: "Class members in a fixed order of kinds"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "reordering a class body moves every member, which the author should accept"
options: { order: { type: "string[]", default: ["static-field", "static-block", "static-method", "private-field", "public-field", "constructor", "private-method", "public-method"], description: "the eight groups, in the order they come" } }
stability: preview
ported_from: [eslint-plugin-unicorn/consistent-class-member-order]
since: 0.5.0
---

A class's members come in groups — by default static fields, static blocks, static methods, private fields, public fields, the constructor, private methods, public methods (`#name` and TypeScript `private` are private; accessors are methods). The first member that comes after a later group is reported, once per class. The suggestion sorts the body (stable within a group, members separated by a blank line) when each member starts its own line and none has a decorator, a computed key or a comment, and a static field and a static block would not swap.

**Why:** One order lets a reader find a class's state, its constructor and its methods without searching.

```codelaw
native: js/consistent-class-member-order
```

## Examples

### JavaScript

#### Don't

```js
class Account {
  deposit() {}

  balance = 0;
}
```

#### Do

```js
class Account {
  balance = 0;

  deposit() {}
}
```

#### Fixed

```js
class Account {
  balance = 0;

  deposit() {}
}
```


### JSX

#### Don't

```jsx
class Account {
  deposit() {}

  balance = 0;
}
```

#### Do

```jsx
class Account {
  balance = 0;

  deposit() {}
}
```

#### Fixed

```jsx
class Account {
  balance = 0;

  deposit() {}
}
```


### TypeScript

#### Don't

```ts
class Account {
  deposit() {}

  balance = 0;
}
```

#### Do

```ts
class Account {
  balance = 0;

  deposit() {}
}
```

#### Fixed

```ts
class Account {
  balance = 0;

  deposit() {}
}
```


### TSX

#### Don't

```tsx
class Account {
  deposit() {}

  balance = 0;
}
```

#### Do

```tsx
class Account {
  balance = 0;

  deposit() {}
}
```

#### Fixed

```tsx
class Account {
  balance = 0;

  deposit() {}
}
```
