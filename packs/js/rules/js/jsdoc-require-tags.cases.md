# js/jsdoc-require-tags — cases

### case: silent without the option

```js
/** Runs the job. */
function run() {}
```

```expect
(none)
```

### case: a missing tag

```js options={"tags":["since"]}
/** Runs the job. */
function run() {}
```

```expect
1:1 js/jsdoc-require-tags
```

### case: the tag is there

```js options={"tags":["since"]}
/**
 * Runs the job.
 * @since 1.2.0
 */
function run() {}
```

```expect
(none)
```

### case: two missing tags

```js options={"tags":["since","public"]}
/** Runs. */
function run() {}
```

```expect
1:1 js/jsdoc-require-tags
1:1 js/jsdoc-require-tags
```

### case: a tag for one kind of code

```js options={"tagContexts":{"returns":"function_declaration"}}
/** A. */
function a() {}
/** B. */
class B {}
```

```expect
1:1 js/jsdoc-require-tags
```
