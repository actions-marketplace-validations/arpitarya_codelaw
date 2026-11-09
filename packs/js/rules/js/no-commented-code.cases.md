# js/no-commented-code — cases

### case: a line of code

```js
const x = 1;
// const y = compute(x);
use(x);
```

```expect
2:1 js/no-commented-code
```

```fixed
const x = 1;
use(x);
```

### case: prose

```js
// This is a note about the value.
const x = 1;
```

```expect
(none)
```

### case: a call without a semicolon is a note

```js
// refresh()
const x = 1;
```

```expect
(none)
```

### case: a call with a semicolon is code

```js
// refresh();
const x = 1;
```

```expect
1:1 js/no-commented-code
```

```fixed
const x = 1;
```

### case: a block comment

```js
/* if (ready) { start(); } */
const x = 1;
```

```expect
1:1 js/no-commented-code
```

```fixed
const x = 1;
```

### case: adjacent lines are one comment

```js
// if (ready) {
//   start();
// }
const x = 1;
```

```expect
1:1 js/no-commented-code
```

```fixed
const x = 1;
```

### case: a documentation example

```js
// e.g. refresh();
const x = 1;
```

```expect
(none)
```

### case: a lone brace

```js
function f() {
  work();
  // }
}
```

```expect
(none)
```

### case: a bare return

```js
function f() {
  // return;
  work();
}
```

```expect
(none)
```

### case: a trailing comment is deleted alone

```js
const x = 1; // const y = 2;
```

```expect
1:14 js/no-commented-code
```

```fixed
const x = 1; 
```

### case: a name

```js
// helper
const x = 1;
```

```expect
(none)
```

### case: a TypeScript declaration

```ts
// const n: number = 1;
const x = 1;
```

```expect
1:1 js/no-commented-code
```

```fixed
const x = 1;
```

### case: a licence header

```js
/*
 * Copyright 2026 Example Ltd. All rights reserved.
 */
const x = 1;
```

```expect
(none)
```
