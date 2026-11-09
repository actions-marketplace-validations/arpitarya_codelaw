# js/jsdoc-informative-docs — cases

### case: a description that repeats the name

```js
/**
 * The name.
 */
function name() {}
```

```expect
1:1 js/jsdoc-informative-docs
```

### case: camel case is split

```js
/** Get user name. */
function getUserName() {}
```

```expect
1:1 js/jsdoc-informative-docs
```

### case: a tag description that repeats

```js
/**
 * Greets.
 * @param {string} name the name
 */
function greet(name) {}
```

```expect
3:4 js/jsdoc-informative-docs
```

### case: a tag description with a dash

```js
/**
 * Greets someone.
 * @param {string} name - The name.
 */
function greet(name) {}
```

```expect
3:4 js/jsdoc-informative-docs
```

### case: a description that says more

```js
/**
 * Opens the connection and retries twice.
 * @param {string} host the server to dial
 */
function connect(host) {}
```

```expect
(none)
```

### case: a method also carries its class's name

```js
class Cache {
  /** The cache size. */
  size() {}
}
```

```expect
2:3 js/jsdoc-informative-docs
```

### case: excluded_tags

```js options={"excluded_tags":["param"]}
/**
 * Greets someone.
 * @param {string} name the name
 */
function greet(name) {}
```

```expect
(none)
```
