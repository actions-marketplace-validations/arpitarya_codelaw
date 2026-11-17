# js/jsdoc-no-missing-syntax — cases

### case: silent without the option

```js
export function run() {}
```

```expect
(none)
```

### case: a block that is missing

```js options={"entries":{"file":{"tag":"file"}}}
export function run() {}
```

```expect
1:1 js/jsdoc-no-missing-syntax
```

### case: the block is there

```js options={"entries":{"file":{"tag":"file"}}}
/** @file Runs the job. */
export function run() {}
```

```expect
(none)
```

### case: a context

```js options={"contexts":["class_declaration"]}
/** Runs. */
function run() {}
class K {}
```

```expect
1:1 js/jsdoc-no-missing-syntax
```

### case: a minimum

```js options={"entries":{"fns":{"context":"function_declaration","minimum":3}}}
/** One. */
function a() {}
/** Two. */
function b() {}
```

```expect
1:1 js/jsdoc-no-missing-syntax
```

### case: a text the block must match

```js options={"entries":{"text":{"matches":"\\bjob\\b"}}}
/** Runs the job. */
function run() {}
```

```expect
(none)
```
