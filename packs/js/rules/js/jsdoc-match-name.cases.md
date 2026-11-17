# js/jsdoc-match-name — cases

### case: silent without the option

```js
/**
 * @param {string} user_name
 */
function find(user_name) {}
```

```expect
(none)
```

### case: a name that is not allowed

```js options={"match":{"camel":{"allowName":"^[a-z][A-Za-z0-9]*$","tags":["param"]}}}
/**
 * @param {string} user_name
 */
function find(user_name) {}
```

```expect
2:20 js/jsdoc-match-name
```

### case: a disallowed name with a replacement

```js options={"match":{"p":{"disallowName":"^_","replacement":"","tags":["param"]}}}
/**
 * @param {string} _id
 * @param {string} [_name]
 */
function find(_id, _name) {}
```

```expect
2:20 js/jsdoc-match-name
3:21 js/jsdoc-match-name
```

```fixed
/**
 * @param {string} id
 * @param {string} [name]
 */
function find(_id, _name) {}
```

### case: a context

```js options={"match":{"p":{"disallowName":"^_","tags":["param"],"context":"class_declaration"}}}
/**
 * @param {string} _a
 */
function f(_a) {}
/**
 * @param {string} _b
 */
class K {}
```

```expect
6:20 js/jsdoc-match-name
```

### case: another tag is not read

```js options={"match":{"p":{"disallowName":"^_","tags":["param"]}}}
/**
 * @returns {string} _x
 */
function f() {}
```

```expect
(none)
```
