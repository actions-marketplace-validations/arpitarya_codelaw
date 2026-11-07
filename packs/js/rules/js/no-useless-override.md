---
id: js/no-useless-override
title: "No method override that only calls super"
languages: [javascript, jsx, typescript, tsx]
fix: auto
fix_reason: "Deletes an override stub someone may be about to fill in, so save and the agent hook leave it for codelaw fix."
on_edit: false
stability: preview
ported_from: [eslint-plugin-unicorn/no-useless-override]
since: 0.5.0
---

A class method that only returns the parent's method of the same name, called with the same arguments, is removed: the class inherits it.

**Why:** The override adds a call and lines and changes nothing.

```codelaw
match:
  kind: method_definition
  any:
    - pattern: { context: "class A { $N() { return super.$N(); } }", selector: method_definition }
    - pattern: { context: "class A { $N($P) { return super.$N($P); } }", selector: method_definition }
    - pattern: { context: "class A { $N($P, $Q) { return super.$N($P, $Q); } }", selector: method_definition }
message: "`$N` only calls `super.$N`; remove it"
fix:
  - remove
```

## Examples

### JavaScript

#### Don't

```js
class Admin extends User {
  greet(name) {
    return super.greet(name);
  }

  ban(user) {
    user.banned = true;
  }
}
```

#### Do

```js
class Admin extends User {

  ban(user) {
    user.banned = true;
  }
}
```


### JSX

#### Don't

```jsx
class Admin extends User {
  greet(name) {
    return super.greet(name);
  }

  ban(user) {
    user.banned = true;
  }
}
```

#### Do

```jsx
class Admin extends User {

  ban(user) {
    user.banned = true;
  }
}
```


### TypeScript

#### Don't

```ts
class Admin extends User {
  greet(name) {
    return super.greet(name);
  }

  ban(user) {
    user.banned = true;
  }
}
```

#### Do

```ts
class Admin extends User {

  ban(user) {
    user.banned = true;
  }
}
```


### TSX

#### Don't

```tsx
class Admin extends User {
  greet(name) {
    return super.greet(name);
  }

  ban(user) {
    user.banned = true;
  }
}
```

#### Do

```tsx
class Admin extends User {

  ban(user) {
    user.banned = true;
  }
}
```
