### case: the number on the left

```js
if (27 !== e.keyCode) close();
```

```expect
1:12 js/prefer-keyboard-event-key
```

```fixed
if ("Escape" !== e.key) close();
```

### case: a letter's code is rewritten by hand

```js
if (e.keyCode === 65) selectAll();
```

```expect
1:5 js/prefer-keyboard-event-key
```
