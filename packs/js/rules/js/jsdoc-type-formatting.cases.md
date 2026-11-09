# js/jsdoc-type-formatting — cases

### case: union spacing

```js
/**
 * @param {string|number} a
 */
function f(a) {}
```

```expect
2:12 js/jsdoc-type-formatting
```

```fixed
/**
 * @param {string | number} a
 */
function f(a) {}
```

### case: an array generic

```js
/**
 * @param {Array<string>} a
 */
function f(a) {}
```

```expect
2:12 js/jsdoc-type-formatting
```

```fixed
/**
 * @param {string[]} a
 */
function f(a) {}
```

### case: an array generic of a union

```js
/**
 * @param {Array<string|number>} a
 */
function f(a) {}
```

```expect
2:12 js/jsdoc-type-formatting
```

```fixed
/**
 * @param {(string | number)[]} a
 */
function f(a) {}
```

### case: a dotted generic

```js
/**
 * @param {Map.<string, number>} a
 */
function f(a) {}
```

```expect
2:12 js/jsdoc-type-formatting
```

```fixed
/**
 * @param {Map<string, number>} a
 */
function f(a) {}
```

### case: semicolons in an object

```js
/**
 * @param {{a: string; b: number}} a
 */
function f(a) {}
```

```expect
2:12 js/jsdoc-type-formatting
```

```fixed
/**
 * @param {{a: string, b: number}} a
 */
function f(a) {}
```

### case: single quotes

```js
/**
 * @param {'a'} a
 */
function f(a) {}
```

```expect
2:12 js/jsdoc-type-formatting
```

```fixed
/**
 * @param {"a"} a
 */
function f(a) {}
```

### case: padding inside the braces

```js
/**
 * @param { string } a
 */
function f(a) {}
```

```expect
2:12 js/jsdoc-type-formatting
```

```fixed
/**
 * @param {string} a
 */
function f(a) {}
```

### case: a function type

```js
/**
 * @param {function(string,number):boolean} a
 */
function f(a) {}
```

```expect
2:12 js/jsdoc-type-formatting
```

```fixed
/**
 * @param {function(string, number): boolean} a
 */
function f(a) {}
```

### case: an arrow type

```js
/**
 * @param {(a:string)=>void} a
 */
function f(a) {}
```

```expect
2:12 js/jsdoc-type-formatting
```

```fixed
/**
 * @param {(a: string) => void} a
 */
function f(a) {}
```

### case: a quoted key that is a name

```js
/**
 * @param {{"a": string}} a
 */
function f(a) {}
```

```expect
2:12 js/jsdoc-type-formatting
```

```fixed
/**
 * @param {{a: string}} a
 */
function f(a) {}
```

### case: already formatted

```js
/**
 * @param {Map<string, number[]>} a
 * @param {{a?: string, "b-c": number}} b
 * @param {[string, number]} c
 * @returns {Promise<void>}
 */
function f(a, b, c) {}
```

```expect
(none)
```

### case: a type over several lines is left alone

```js
/**
 * @param {{
 *   a: string;
 *   b: number
 * }} a
 */
function f(a) {}
```

```expect
(none)
```

### case: a tag the rule does not read

```js
/**
 * @callback Fn
 * @template {string|number} T
 */
function f() {}
```

```expect
(none)
```

### case: a conditional type is kept as written

```js
/**
 * @param {T extends U ? X : Y} a
 */
function f(a) {}
```

```expect
(none)
```
