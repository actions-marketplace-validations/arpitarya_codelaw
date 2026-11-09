---
id: js/no-undeclared-class-members
title: "No `this.x` the class does not declare"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "declaring the field adds a member every instance has, which the author should accept"
stability: preview
ported_from: [eslint-plugin-unicorn/no-undeclared-class-members]
since: 0.5.0
---

In a class with no `extends`, every `this.name` a method reads, writes or calls names a member the class declares: a field, a method, an accessor, a TypeScript parameter property, or a `this.name =` in the constructor. A typo, or state invented in some method, is reported. The suggestion, for a plain `this.name = value` outside the constructor, declares `name;` as a field above the first member.

**Why:** A member that only appears in one method's body is easy to misspell and hard to find; declaring it lists the object's shape in one place.

```codelaw
native: js/no-undeclared-class-members
```

## Examples

### JavaScript

#### Don't

```js
class Timer {
  start() {
    this.id = setTimeout(tick, 10);
  }
}
```

#### Do

```js
class Timer {
  id;
  start() {
    this.id = setTimeout(tick, 10);
  }
}
```

#### Fixed

```js
class Timer {
  id;
  start() {
    this.id = setTimeout(tick, 10);
  }
}
```


### JSX

#### Don't

```jsx
class Timer {
  start() {
    this.id = setTimeout(tick, 10);
  }
}
```

#### Do

```jsx
class Timer {
  id;
  start() {
    this.id = setTimeout(tick, 10);
  }
}
```

#### Fixed

```jsx
class Timer {
  id;
  start() {
    this.id = setTimeout(tick, 10);
  }
}
```


### TypeScript

#### Don't

```ts
class Timer {
  start() {
    this.id = setTimeout(tick, 10);
  }
}
```

#### Do

```ts
class Timer {
  id;
  start() {
    this.id = setTimeout(tick, 10);
  }
}
```

#### Fixed

```ts
class Timer {
  id;
  start() {
    this.id = setTimeout(tick, 10);
  }
}
```


### TSX

#### Don't

```tsx
class Timer {
  start() {
    this.id = setTimeout(tick, 10);
  }
}
```

#### Do

```tsx
class Timer {
  id;
  start() {
    this.id = setTimeout(tick, 10);
  }
}
```

#### Fixed

```tsx
class Timer {
  id;
  start() {
    this.id = setTimeout(tick, 10);
  }
}
```
