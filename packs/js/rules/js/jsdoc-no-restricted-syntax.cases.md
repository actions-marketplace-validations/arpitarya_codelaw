# js/jsdoc-no-restricted-syntax — cases

### case: silent without the option

```js
/** @type {string} */
const name = "x";
```

```expect
(none)
```

### case: a tag

```js options={"entries":{"typeTag":{"tag":"type"}}}
/** @type {string} */
const name = "x";
/** The age. */
const age = 1;
```

```expect
1:1 js/jsdoc-no-restricted-syntax
```

### case: a context

```js options={"contexts":["function_declaration"]}
/** A. */
function a() {}
/** B. */
class B {}
```

```expect
1:1 js/jsdoc-no-restricted-syntax
```

### case: a text

```js options={"entries":{"old":{"matches":"Deprecated:"}}}
/** Deprecated: use b. */
function a() {}
```

```expect
1:1 js/jsdoc-no-restricted-syntax
```
