---
id: js/prefer-class-fields
title: "A class field for a constant the constructor assigns first"
languages: [javascript, jsx, typescript, tsx]
fix: auto
stability: preview
ported_from: [eslint-plugin-unicorn/prefer-class-fields]
since: 0.5.0
---

When the first statement of a class's constructor is `this.name = <literal>` (a string, number, boolean, `null`, regular expression or plain template), the value belongs in a field declaration. The fix removes the assignment and declares `name = <literal>;` at the end of the class body, or gives an existing bare field the value. When the field already has a value, the change is a suggestion. A TypeScript parameter property of the same name, two fields of that name, a `declare`, `abstract` or `!` field, a method of that name, or a comment in the statement leave it reported without a fix. Statements after `super()` are not first, so a subclass's are not reported.

**Why:** A field declaration shows the object's shape and default at a glance, without reading the constructor.

```codelaw
native: js/prefer-class-fields
```

## Examples

### JavaScript

#### Don't

```js
class Counter {
  constructor() {
    this.count = 0;
  }
}
```

#### Do

```js
class Counter {
  count = 0;
}
```

#### Fixed

```js
class Counter {
  constructor() {
  }
  count = 0;
}
```


### JSX

#### Don't

```jsx
class Counter {
  constructor() {
    this.count = 0;
  }
}
```

#### Do

```jsx
class Counter {
  count = 0;
}
```

#### Fixed

```jsx
class Counter {
  constructor() {
  }
  count = 0;
}
```


### TypeScript

#### Don't

```ts
class Counter {
  constructor() {
    this.count = 0;
  }
}
```

#### Do

```ts
class Counter {
  count = 0;
}
```

#### Fixed

```ts
class Counter {
  constructor() {
  }
  count = 0;
}
```


### TSX

#### Don't

```tsx
class Counter {
  constructor() {
    this.count = 0;
  }
}
```

#### Do

```tsx
class Counter {
  count = 0;
}
```

#### Fixed

```tsx
class Counter {
  constructor() {
  }
  count = 0;
}
```
