---
id: js/no-this-in-static
title: "The class name, not this, in a static method"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "naming the class is not the same as `this` when a subclass calls the method; review it"
stability: preview
ported_from: [biome/complexity/noThisInStatic]
since: 0.5.0
---

A static method of a named class refers to the class by its name, not by `this`.

**Why:** `this` in a static method is the class only when called on it; called on a subclass or detached it is something else, which is rarely what the code means.

```codelaw
match:
  kind: this
  inside:
    kind: method_definition
    regex: '^((@\S+|public|private|protected|override)\s+)*static\b'
    stopBy: { any: [{kind: function_declaration}, {kind: function_expression}, {kind: method_definition}, {kind: generator_function_declaration}, {kind: generator_function}] }
    inside:
      kind: class_body
      inside: { kind: class_declaration, has: { field: name, pattern: $CLASS } }
message: "Use `$CLASS` instead of `this` in a static method"
fix: "$CLASS"
```

## Examples

### JavaScript

#### Don't

```js
class Config {
  static load() {
    return this.defaults;
  }
}
```

#### Do

```js
class Config {
  static load() {
    return Config.defaults;
  }
}
```

### JSX

#### Don't

```jsx
class Config {
  static load() {
    return this.defaults;
  }
}
```

#### Do

```jsx
class Config {
  static load() {
    return Config.defaults;
  }
}
```

### TypeScript

#### Don't

```ts
class Config {
  static load() {
    return this.defaults;
  }
}
```

#### Do

```ts
class Config {
  static load() {
    return Config.defaults;
  }
}
```

### TSX

#### Don't

```tsx
class Config {
  static load() {
    return this.defaults;
  }
}
```

#### Do

```tsx
class Config {
  static load() {
    return Config.defaults;
  }
}
```
