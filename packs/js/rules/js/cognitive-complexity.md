---
id: js/cognitive-complexity
title: "A function's cognitive complexity is at most 15"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which parts to extract, and what to name them, is the author's"
fix_exception: restructure
options: { threshold: { type: "integer", default: 15, description: "the highest cognitive complexity a function may have" } }
stability: preview
ported_from: [eslint-plugin-sonarjs/cognitive-complexity]
since: 0.5.0
---

Cognitive complexity (SonarSource's measure) adds one for each break in the linear flow — `if`, `else if`, `else`, `switch`, a loop, `catch`, `?:`, a labelled `break` or `continue`, each run of `&&`, `||` or `??` and each change of operator in it — and adds the nesting depth for the structures that nest. A function over `threshold` (15) is reported; functions nested in it count with it.

**Why:** Nesting costs a reader more than length; the measure tracks that.

```codelaw
native: js/cognitive-complexity
```

## How to fix

1. Extract nested blocks into functions with names.
2. Return early instead of nesting the main path inside `if`s.

## Examples

### JavaScript

#### Don't

```js options={"threshold":5}
function f(a, b, c) {
  if (a) {
    for (const x of b) {
      if (x) {
        while (c) {
          if (x && c || a) {
            c = c.next;
          }
        }
      }
    }
  }
}
```

#### Do

```js options={"threshold":5}
function f(a, b) {
  if (a) {
    use(b);
  }
}
```


### JSX

#### Don't

```jsx options={"threshold":5}
function f(a, b, c) {
  if (a) {
    for (const x of b) {
      if (x) {
        while (c) {
          if (x && c || a) {
            c = c.next;
          }
        }
      }
    }
  }
}
```

#### Do

```jsx options={"threshold":5}
function f(a, b) {
  if (a) {
    use(b);
  }
}
```


### TypeScript

#### Don't

```ts options={"threshold":5}
function f(a, b, c) {
  if (a) {
    for (const x of b) {
      if (x) {
        while (c) {
          if (x && c || a) {
            c = c.next;
          }
        }
      }
    }
  }
}
```

#### Do

```ts options={"threshold":5}
function f(a, b) {
  if (a) {
    use(b);
  }
}
```


### TSX

#### Don't

```tsx options={"threshold":5}
function f(a, b, c) {
  if (a) {
    for (const x of b) {
      if (x) {
        while (c) {
          if (x && c || a) {
            c = c.next;
          }
        }
      }
    }
  }
}
```

#### Do

```tsx options={"threshold":5}
function f(a, b) {
  if (a) {
    use(b);
  }
}
```
