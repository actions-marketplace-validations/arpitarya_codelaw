# js/prefer-string-pad-start-end — cases

### case: padEnd

```js
const cell = name + " ".repeat(8 - name.length);
```

```expect
1:14 js/prefer-string-pad-start-end
```

```fixed
const cell = name.padEnd(8, " ");
```

### case: a longer pad stays

```js
const cell = "ab".repeat(10 - name.length) + name;
```

```expect
(none)
```
