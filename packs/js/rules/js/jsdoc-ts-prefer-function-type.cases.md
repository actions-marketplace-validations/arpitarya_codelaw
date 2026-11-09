# js/jsdoc-ts-prefer-function-type — cases

### case: a lone call signature

```js
/**
 * @param {{(a: string): void}} x
 */
function f(x) {}
```

```expect
2:12 js/jsdoc-ts-prefer-function-type
```

```fixed
/**
 * @param {(a: string) => void} x
 */
function f(x) {}
```

### case: in a union it is parenthesised

```js
/**
 * @param {{(a: string): void} | null} x
 */
function f(x) {}
```

```expect
2:12 js/jsdoc-ts-prefer-function-type
```

```fixed
/**
 * @param {((a: string) => void) | null} x
 */
function f(x) {}
```

### case: a call signature with a property

```js
/**
 * @param {{(a: string): void, name: string}} x
 */
function f(x) {}
```

```expect
(none)
```

### case: a construct signature

```js
/**
 * @param {{new (a: string): Foo}} x
 */
function f(x) {}
```

```expect
(none)
```

### case: already a function type

```js
/**
 * @param {(a: string) => void} x
 */
function f(x) {}
```

```expect
(none)
```
