---
id: js/class-methods-use-this
title: "A class method uses this"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "making the method static changes how callers reach it; the author decides"
fix_exception: restructure
stability: preview
ported_from: [eslint/class-methods-use-this]
since: 0.5.0
---

An instance method of a class — not the constructor, not `static` — uses `this` or `super` somewhere in its body (an arrow function inside counts; a nested `function` does not).

**Why:** A method that never touches its instance is a function in a class's clothing; as `static`, or a plain function, it says it needs no instance.

```codelaw
utils:
  boundary:
    any:
      - kind: function_expression
      - kind: function_declaration
      - kind: generator_function
      - kind: generator_function_declaration
      - kind: class_declaration
      - kind: class
match:
  kind: method_definition
  inside: { kind: class_body }
  has: { field: body, kind: statement_block }
  not:
    any:
      - regex: '^(?:@[^\n]*\s+)*(?:(?:public|private|protected|readonly|async|override|abstract|declare)\s+)*static\b'
      - has: { field: name, regex: '^constructor$' }
      - has: { field: body, has: { stopBy: { matches: boundary }, any: [{ kind: this }, { kind: super }] } }
message: "This method doesn't use `this`; make it static or a function"
```

## How to fix

1. Mark the method `static` and call it on the class, or move it out of the class as a function.

## Examples

### JavaScript

#### Don't

```js
class Price {
  format(value) {
    return `$${value.toFixed(2)}`;
  }
}
```

#### Do

```js
class Price {
  static format(value) {
    return `$${value.toFixed(2)}`;
  }
}
```


### JSX

#### Don't

```jsx
class Price {
  format(value) {
    return `$${value.toFixed(2)}`;
  }
}
```

#### Do

```jsx
class Price {
  static format(value) {
    return `$${value.toFixed(2)}`;
  }
}
```


### TypeScript

#### Don't

```ts
class Price {
  format(value) {
    return `$${value.toFixed(2)}`;
  }
}
```

#### Do

```ts
class Price {
  static format(value) {
    return `$${value.toFixed(2)}`;
  }
}
```


### TSX

#### Don't

```tsx
class Price {
  format(value) {
    return `$${value.toFixed(2)}`;
  }
}
```

#### Do

```tsx
class Price {
  static format(value) {
    return `$${value.toFixed(2)}`;
  }
}
```
