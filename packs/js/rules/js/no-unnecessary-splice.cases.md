# js/no-unnecessary-splice — cases

### case: unshift

```js
list.splice(0, 0, a, b);
```

```expect
1:1 js/no-unnecessary-splice
```

```fixed
list.unshift(a, b);
```

### case: pop

```js
this.items.splice(this.items.length - 1, 1);
```

```expect
1:1 js/no-unnecessary-splice
```

```fixed
this.items.pop();
```

### case: a used result stays

```js
const [first] = list.splice(0, 1);
```

```expect
(none)
```

### case: a middle index stays

```js
list.splice(2, 1);
```

```expect
(none)
```
