# js/jsdoc-require-template — cases

### case: a generic type alias

```ts
/** A pair. */
type Pair<A, B> = [A, B];
```

```expect
1:1 js/jsdoc-require-template
```

### case: a non-generic function needs none

```ts
/** Ticks. */
function tick(): void {}
```

```expect

```
