---
id: js/grouped-accessor-pairs
title: "A getter and its setter sit together"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the setter moves below its getter; a comment or decorator above the setter stays where it was, and a class whose members are read in order (a decorator that lists them) changes"
stability: preview
ported_from: [eslint/grouped-accessor-pairs]
since: 0.5.0
---

A getter and the setter of the same property are next to each other.

**Why:** One property split across a class is easy to change in one place and forget in the other.

```codelaw
match:
  kind: method_definition
  regex: ^(static\s+)?set\s
  has:
    field: name
    pattern: $N
  not:
    follows:
      kind: method_definition
      regex: ^(static\s+)?get\s
      has:
        field: name
        pattern: $N
  any:
  - follows:
      stopBy: end
      kind: method_definition
      regex: ^(static\s+)?get\s
      pattern: $G
      has:
        field: name
        pattern: $N
  - precedes:
      stopBy: end
      kind: method_definition
      regex: ^(static\s+)?get\s
      pattern: $G
      has:
        field: name
        pattern: $N
  all:
  - not:
      precedes:
        kind: method_definition
        regex: ^(static\s+)?get\s
        has:
          field: name
          pattern: $N
message: Put the setter of `$N` next to its getter
fix:
  - move: { after: $G }
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
class User {
  get name() {
    return this.n;
  }
  save() {}
  set name(v) {
    this.n = v;
  }
}
```

#### Do

```js
class User {
  get name() {
    return this.n;
  }
  set name(v) {
    this.n = v;
  }
  save() {}
}
```

### JSX

#### Don't

```jsx
class User {
  get name() {
    return this.n;
  }
  save() {}
  set name(v) {
    this.n = v;
  }
}
```

#### Do

```jsx
class User {
  get name() {
    return this.n;
  }
  set name(v) {
    this.n = v;
  }
  save() {}
}
```

### TypeScript

#### Don't

```ts
class User {
  get name() {
    return this.n;
  }
  save() {}
  set name(v) {
    this.n = v;
  }
}
```

#### Do

```ts
class User {
  get name() {
    return this.n;
  }
  set name(v) {
    this.n = v;
  }
  save() {}
}
```

### TSX

#### Don't

```tsx
class User {
  get name() {
    return this.n;
  }
  save() {}
  set name(v) {
    this.n = v;
  }
}
```

#### Do

```tsx
class User {
  get name() {
    return this.n;
  }
  set name(v) {
    this.n = v;
  }
  save() {}
}
```
