# js/jsdoc-check-template-names — cases

### case: an unused template

```js
/**
 * @template T
 * @template U
 * @param {T} a
 * @returns {T}
 */
function f(a) {}
```

```expect
3:4 js/jsdoc-check-template-names
```

### case: used in a typedef

```js
/**
 * @template T
 * @typedef {Array<T>} List
 */
```

```expect
(none)
```

### case: used in the return type only

```js
/**
 * @template T
 * @returns {Promise<T>}
 */
function f() {}
```

```expect
(none)
```

### case: used in a constraint

```js
/**
 * @template T
 * @template {keyof T} K
 * @param {K} key
 */
function f(key) {}
```

```expect
(none)
```

### case: a list of names

```js
/**
 * @template T, U
 * @param {T} a
 */
function f(a) {}
```

```expect
2:4 js/jsdoc-check-template-names
```

### case: a default that uses another name

```js
/**
 * @template T
 * @template [U=T[]]
 * @param {U} a
 */
function f(a) {}
```

```expect
(none)
```

### case: a callback

```js
/**
 * @callback Mapper
 * @template T
 * @param {string} s
 */
```

```expect
3:4 js/jsdoc-check-template-names
```

### case: a class whose method uses the name

```js
/**
 * @template T
 */
class Box {
  /**
   * @param {T} v
   */
  set(v) {}
}
```

```expect
(none)
```
