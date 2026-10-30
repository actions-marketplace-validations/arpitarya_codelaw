# js/no-unused-function-parameters — cases

### case: a used last parameter

```js
const f = (a, b) => a + b;
```

```expect

```

### case: an underscore parameter

```js
function f(a, _b) { return a; }
```

```expect

```

### case: a single bare arrow parameter

```js
const f = e => 1;
```

```expect
1:11 js/no-unused-function-parameters
```

```fixed
const f = _e => 1;
```

### case: a TypeScript parameter property is a field

```ts
class A {
  constructor(private readonly store: Store) {}
}
```

```expect

```

### case: a declaration without a body

```ts
declare function f(a: number): void;
```

```expect

```

### case: a function type's parameter

```ts
export function on(cb: (event: Event) => void) {
  return cb;
}
```

```expect

```
