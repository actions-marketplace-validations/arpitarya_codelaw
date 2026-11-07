# js/prefer-unicode-code-point-escapes — cases

### case: a surrogate pair

```js
const pile = "\uD83D\uDCA9";
```

```expect
1:14 js/prefer-unicode-code-point-escapes
```

```fixed
const pile = "\u{1F4A9}";
```

### case: a template

```js
const t = `\u2661${x}`;
```

```expect
1:11 js/prefer-unicode-code-point-escapes
```

```fixed
const t = `\u{2661}${x}`;
```
