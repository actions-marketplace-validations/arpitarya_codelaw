# js/comment-type — cases

### case: a block of several lines

```js
function f() {
  /*
   * one
   * two
   */
  g();
}
```

```expect
2:3 js/comment-type
```

```fixed
function f() {
  // one
  // two
  g();
}
```

### case: a doc comment and line comments are allowed

```js
/** The total. */
const total = 1;
// a note
```

```expect
```

### case: a directive is left alone

```js
/* eslint-disable no-console */
/*! licence */
const x = /*#__PURE__*/ make();
```

```expect
```

### case: a comment between code on its line is left alone

```js
f(/* x */ 1);
```

```expect
```

### case: a block with code after it

```js
/* head */ g();
```

```expect
1:1 js/comment-type
```

```fixed
// head
g();
```

### case: only multiline is allowed

```js
// one
// two
g();
```

```expect
```

### case: a comment in JSX braces is left alone

```jsx
const a = <div>{/* note */}</div>;
```

```expect
```
