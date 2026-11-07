---
id: js/max-nested-callbacks
title: "No callbacks nested more than ten deep"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which callbacks become named functions or awaits is the author's"
fix_exception: restructure
stability: preview
ported_from: [eslint/max-nested-callbacks]
since: 0.5.0
---

A function passed as an argument is not nested inside more than ten others (ESLint's default `max: 10`).

**Why:** Past that depth the code is a pyramid no reader can hold; promises, `async`/`await` or named functions flatten it.

```codelaw
utils:
  cb:
    any: [{ kind: arrow_function }, { kind: function_expression }]
    inside: { kind: arguments }
match:
  matches: cb
  inside:
    stopBy: end
    matches: cb
    inside:
      stopBy: end
      matches: cb
      inside:
        stopBy: end
        matches: cb
        inside:
          stopBy: end
          matches: cb
          inside:
            stopBy: end
            matches: cb
            inside:
              stopBy: end
              matches: cb
              inside:
                stopBy: end
                matches: cb
                inside:
                  stopBy: end
                  matches: cb
                  inside:
                    stopBy: end
                    matches: cb
                    inside: { stopBy: end, matches: cb }
message: "Callbacks are nested more than ten deep"
```

## How to fix

1. Turn the chain into `async`/`await`, or give the inner callbacks names and define them at the top level.

## Examples

### JavaScript

#### Don't

```js
a(() => b(() => c(() => d(() => e(() => f(() => g(() => h(() => i(() => j(() => k(() => {})))))))))));
```

#### Do

```js
a(() => b(() => c(() => d(() => e(() => f(() => g(() => h(() => i(() => j(() => {}))))))))));
```


### JSX

#### Don't

```jsx
a(() => b(() => c(() => d(() => e(() => f(() => g(() => h(() => i(() => j(() => k(() => {})))))))))));
```

#### Do

```jsx
a(() => b(() => c(() => d(() => e(() => f(() => g(() => h(() => i(() => j(() => {}))))))))));
```


### TypeScript

#### Don't

```ts
a(() => b(() => c(() => d(() => e(() => f(() => g(() => h(() => i(() => j(() => k(() => {})))))))))));
```

#### Do

```ts
a(() => b(() => c(() => d(() => e(() => f(() => g(() => h(() => i(() => j(() => {}))))))))));
```


### TSX

#### Don't

```tsx
a(() => b(() => c(() => d(() => e(() => f(() => g(() => h(() => i(() => j(() => k(() => {})))))))))));
```

#### Do

```tsx
a(() => b(() => c(() => d(() => e(() => f(() => g(() => h(() => i(() => j(() => {}))))))))));
```
