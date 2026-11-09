# js/require-atomic-updates — cases

### case: foo = foo + await

```js
let foo;
async function x() { foo = foo + await amount; }
```

```expect
2:22 js/require-atomic-updates
```

### case: a read after the await is fresh

```js
let foo;
async function x() { foo = await bar + foo; }
```

```expect
(none)
```

### case: no await between the read and the write

```js
let foo;
async function x() { foo = foo + 1; await bar; }
```

```expect
(none)
```

### case: a local nothing else uses

```js
async function x() { let foo; foo += await bar; }
```

```expect
(none)
```

### case: a local a closure uses

```js
async function x() { let foo; bar(() => foo); foo += await amount; }
```

```expect
1:47 js/require-atomic-updates
```

### case: an undeclared name

```js
async function x() { foo += await bar; }
```

```expect
(none)
```

### case: a yield in a generator

```js
let foo;
function* x() { foo += yield baz }
```

```expect
2:17 js/require-atomic-updates
```

### case: the read is in the branch not taken before the await

```js
let foo;
async function x() { foo = condition ? foo : await bar; }
```

```expect
(none)
```

### case: the read is before a branch that awaits

```js
let foo;
async function x() { foo = foo + (bar ? baz : await amount); }
```

```expect
2:22 js/require-atomic-updates
```

### case: a property of a shared object

```js
const foo = {};
async function x() { foo.bar += await baz }
```

```expect
2:22 js/require-atomic-updates
```

### case: a property of a parameter

```js
async function f(foo) {
  let buz = await get(foo.id);
  foo.bar = buz.bar;
}
```

```expect
3:3 js/require-atomic-updates
```

### case: a local object whose property is set

```js
const foo = {};
async function x() { foo.bar = await baz; }
```

```expect
(none)
```

### case: a loop does not carry the read to the next pass

```js
let foo;
async function x() { while (condition) { foo += await amount; } }
```

```expect
2:42 js/require-atomic-updates
```

### case: try and catch

```js
const process = getProcess();
const opts = {};
async () => {
  opts.spec = process.stdin;
  try {
    const { exit_code } = await run(opts);
    process.exitCode = exit_code;
  } catch (e) {
    process.exitCode = 1;
  }
};
```

```expect
7:5 js/require-atomic-updates
9:5 js/require-atomic-updates
```

### case: a handler that only awaits after writing

```js
async function f() {
  try {
    this.foo = doSomething();
  } catch (e) {
    this.foo = null;
    await doElse();
  }
}
```

```expect
(none)
```

### case: a for-of whose right side awaits

```js
let foo;
async function x() { for (foo of foo.concat(await load())) {} }
```

```expect
2:22 js/require-atomic-updates
```
