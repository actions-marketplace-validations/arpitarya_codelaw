# js/consistent-compound-words — cases

### case: inside a longer name

```js
function isInViewPort(el) {}
isInViewPort(a);
```

```expect
1:10 js/consistent-compound-words
```

```fixed
function isInViewport(el) {}
isInViewport(a);
```

### case: a parameter

```js
function run(callBack) { callBack(); }
```

```expect
1:14 js/consistent-compound-words
```

```fixed
function run(callback) { callback(); }
```

### case: a separate word stays

```js
const fileName = "a";
```

```expect
(none)
```

### case: a longer word stays

```js
const timeOuter = 1;
```

```expect
(none)
```
