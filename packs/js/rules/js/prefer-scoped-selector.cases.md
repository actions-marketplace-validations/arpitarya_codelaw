# js/prefer-scoped-selector — cases

### case: a child combinator

```js
const item = list.querySelector("ul > li");
```

```expect
1:14 js/prefer-scoped-selector
```

```fixed
const item = list.querySelector(":scope ul > li");
```

### case: a simple selector stays

```js
const item = list.querySelector(".item");
```

```expect
(none)
```

### case: document stays

```js
const item = document.querySelector(".a .b");
```

```expect
(none)
```

### case: a selector list stays

```js
const item = list.querySelector(".a .b, .c");
```

```expect
(none)
```

### case: an already scoped selector stays

```js
const item = list.querySelector(":scope .a .b");
```

```expect
(none)
```
