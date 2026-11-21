---
id: js/no-using-resource-escape
title: "A `using` resource does not leave the scope that disposes it"
languages: [javascript, jsx]
fix: manual
fix_reason: "whether the caller or the function owns the resource is the application's to decide"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-using-resource-escape]
since: 0.5.2
---

A resource declared with `using` or `await using` is disposed when its scope exits, so returning it, exporting it, or returning or exporting a function that captures it gives later code a disposed resource. Reported: the resource in a `return` or an export (`export { resource }`, `export default`, `export const x = …`), and a function that reads it; also when they sit in an array element, an object property value or method, either branch of a conditional, either side of `&&`, `||` or `??`, or the last operand of a sequence; and a function named in a return or export that the scope declares (a function declaration, or a `const` initialised with a function). Not checked: aliases, destructured or reassigned bindings, classes, calls, awaited expressions, spreads, `yield`, re-exports, and callbacks handed to other functions, whose lifetime is unknown. A function parameter of the same name shadows the resource. Behaviour of eslint-plugin-unicorn 79.0.0's `no-using-resource-escape`, read and rewritten. JavaScript and JSX only: the pinned TypeScript grammar (0.23.2) does not parse `using` (W-190, step 2).

**Why:** The disposer has run by the time the caller sees the resource: a file handle closed, a connection released.

```codelaw
native: js/no-using-resource-escape
```

## How to fix

1. Return what the resource produces, not the resource.
2. Or let the returned function own the resource: declare it inside that function.

## Examples

### JavaScript

#### Don't

```js
function openResource() {
  using resource = acquire();
  return resource;
}
```

#### Do

```js
function readResource() {
  using resource = acquire();
  return resource.read();
}
```

### JSX

#### Don't

```jsx
function openResource() {
  using resource = acquire();
  return resource;
}
```

#### Do

```jsx
function readResource() {
  using resource = acquire();
  return resource.read();
}
```
