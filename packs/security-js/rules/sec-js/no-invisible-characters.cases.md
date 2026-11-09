# sec-js/no-invisible-characters — cases

### case: a string with a filler

```js
const user = "adminㅤ";
```

```expect
1:14 sec-js/no-invisible-characters
```

### case: a name with a filler

```js
const admﾠin = 1;
```

```expect
1:7 sec-js/no-invisible-characters
```

### case: an escape is not the character

```js
const mark = "\u3164";
```

```expect
(none)
```

### case: plain code

```js
const user = "admin";
```

```expect
(none)
```
