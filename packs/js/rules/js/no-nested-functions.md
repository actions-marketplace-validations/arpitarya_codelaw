---
id: js/no-nested-functions
title: "No function nested more than four deep"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which function to pull out and what to name it is the author's"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-sonarjs/no-nested-functions]
since: 0.5.0
---

A function is not nested inside more than four others (SonarJS's default threshold).

**Why:** Each level adds a scope a reader has to hold in mind; past four, the code is hard to follow and to test.

```codelaw
utils:
  fn:
    any:
      - kind: function_declaration
      - kind: function_expression
      - kind: arrow_function
      - kind: method_definition
      - kind: generator_function
      - kind: generator_function_declaration
match:
  matches: fn
  inside:
    stopBy: end
    matches: fn
    inside:
      stopBy: end
      matches: fn
      inside:
        stopBy: end
        matches: fn
        inside: { stopBy: end, matches: fn }
message: "This function is nested more than four deep; extract it"
```

## How to fix

1. Move the inner function out to the top level of the module (or a method), passing what it needs as arguments.

## Examples

### JavaScript

#### Don't

```js
function a() {
  return function b() {
    return function c() {
      return function d() {
        return function e() {};
      };
    };
  };
}
```

#### Do

```js
function e() {}
function a() {
  return function b() {
    return function c() {
      return function d() {
        return e;
      };
    };
  };
}
```


### JSX

#### Don't

```jsx
function a() {
  return function b() {
    return function c() {
      return function d() {
        return function e() {};
      };
    };
  };
}
```

#### Do

```jsx
function e() {}
function a() {
  return function b() {
    return function c() {
      return function d() {
        return e;
      };
    };
  };
}
```


### TypeScript

#### Don't

```ts
function a() {
  return function b() {
    return function c() {
      return function d() {
        return function e() {};
      };
    };
  };
}
```

#### Do

```ts
function e() {}
function a() {
  return function b() {
    return function c() {
      return function d() {
        return e;
      };
    };
  };
}
```


### TSX

#### Don't

```tsx
function a() {
  return function b() {
    return function c() {
      return function d() {
        return function e() {};
      };
    };
  };
}
```

#### Do

```tsx
function e() {}
function a() {
  return function b() {
    return function c() {
      return function d() {
        return e;
      };
    };
  };
}
```
