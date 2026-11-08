# Cases: js/no-inline-comments

### case: a comment on its own line is fine

W-169 (corpus): the rule flagged every `//` comment whose previous sibling was a node, so a commented-out block or a comment between statements was "inline", and moving it ("fixing") swapped comments around without end.

```js
const a = 1;
// about b
const b = 2;
// about c
// and more
const c = 3;
```

```expect
(none)
```

### case: a comment after code is flagged and moved above it

```js
const max = 3; // retries
```

```expect
1:16 js/no-inline-comments
```

```fixed
// retries
const max = 3;
```

### case: a directive comment after code is left alone

```js
const x = 1; // eslint-disable-line no-unused-vars
```

```expect
(none)
```
