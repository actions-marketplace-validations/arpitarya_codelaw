# js/consistent-optional-chaining — cases

### case: a plain read after an optional one under ||

```js
x = foo?.bar || foo.baz;
```

```expect
1:5 js/consistent-optional-chaining
```

```fixed
x = foo?.bar || foo?.baz;
```

### case: an optional read after a plain one under ||

```js
x = foo.bar || foo?.baz;
```

```expect
1:5 js/consistent-optional-chaining
```

```fixed
x = foo.bar || foo.baz;
```

### case: two optional reads under &&

```js
x = foo?.bar && foo?.baz;
```

```expect
1:5 js/consistent-optional-chaining
```

```fixed
x = foo?.bar && foo.baz;
```

### case: a plain then optional read under &&

```js
x = foo.bar && foo?.baz;
```

```expect
1:5 js/consistent-optional-chaining
```

```fixed
x = foo.bar && foo.baz;
```

### case: different bases stay

```js
x = foo?.bar || baz.qux;
```

```expect
(none)
```

### case: the consistent forms stay

```js
x = foo?.bar || foo?.baz;
y = foo.bar && foo.baz;
```

```expect
(none)
```

### case: a truthiness guard is not rewritten

```js
x = foo && foo.bar;
```

```expect
(none)
```
