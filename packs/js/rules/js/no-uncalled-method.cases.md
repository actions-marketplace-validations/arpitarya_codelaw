# js/no-uncalled-method — cases

### case: an unknown receiver

```js
const m = unknown.sort;
```

```expect
(none)
```

### case: bound and called

```js
const array = [];
array.sort.call(array);
```

```expect
(none)
```

### case: a literal in a condition

```js
if ([1].pop) {
}
```

```expect
1:9 js/no-uncalled-method
```

### case: a string method on an array is not this rule's

```js
const array = [];
const f = array.toLowerCase;
```

```expect
(none)
```
