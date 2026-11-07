# js/no-confusing-array-splice — cases

### case: splice as a statement

```js
list.splice(i, 1, value);
```

```expect
1:1 js/no-confusing-array-splice
```

```fixed
list[i] = value;
```

### case: toSpliced

```js
const copy = list.toSpliced(2, 1, value);
```

```expect
1:14 js/no-confusing-array-splice
```

```fixed
const copy = list.with(2, value);
```

### case: a splice whose result is used stays

```js
const removed = list.splice(i, 1, value);
```

```expect
(none)
```

### case: a negative index stays

```js
list.splice(-1, 1, value);
```

```expect
(none)
```

### case: removing more than one stays

```js
list.splice(i, 2, value);
```

```expect
(none)
```
