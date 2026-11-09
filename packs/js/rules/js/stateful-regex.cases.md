# js/stateful-regex — cases

### case: reset

```js
const r = /a/g;
r.test(s1);
r.lastIndex = 0;
r.test(s2);
```

```expect
(none)
```

### case: exec in a loop condition

```js
do {
  step();
} while (new RegExp("x", "g").exec(t));
```

```expect
3:10 js/stateful-regex
```

### case: g with y

```js
const r = /x/gy;
```

```expect
1:11 js/stateful-regex
```
