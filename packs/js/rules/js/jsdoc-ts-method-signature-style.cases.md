# js/jsdoc-ts-method-signature-style — cases

### case: a method signature

```js
/**
 * @param {{foo(a: string): void}} x
 */
function f(x) {}
```

```expect
2:13 js/jsdoc-ts-method-signature-style
```

```fixed
/**
 * @param {{foo: (a: string) => void}} x
 */
function f(x) {}
```

### case: an optional method

```js
/**
 * @param {{foo?(a: string): void}} x
 */
function f(x) {}
```

```expect
2:13 js/jsdoc-ts-method-signature-style
```

```fixed
/**
 * @param {{foo?: (a: string) => void}} x
 */
function f(x) {}
```

### case: overloads have no fix

```js
/**
 * @param {{foo(a: string): void, foo(a: number): void}} x
 */
function f(x) {}
```

```expect
2:13 js/jsdoc-ts-method-signature-style
2:35 js/jsdoc-ts-method-signature-style
```

### case: a function property

```js
/**
 * @param {{foo: (a: string) => void}} x
 */
function f(x) {}
```

```expect
(none)
```

### case: the method style

```js options={"style":"method"}
/**
 * @param {{foo: (a: string) => void}} x
 */
function f(x) {}
```

```expect
2:13 js/jsdoc-ts-method-signature-style
```

```fixed
/**
 * @param {{foo(a: string): void}} x
 */
function f(x) {}
```
