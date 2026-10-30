# js/no-char-at-multichar-compare — cases

### case: escapes count as one character

```js
if (s.charAt(i) === "\n" || s.charAt(i) == "\u00e9" || s.charAt(i) !== "") {}
```

```expect

```

### case: either side

```js
ok = "ab" != s.charAt(0);
```

```expect
1:6 js/no-char-at-multichar-compare
```
