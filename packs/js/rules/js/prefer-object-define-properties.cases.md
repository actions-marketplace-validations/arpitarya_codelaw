# js/prefer-object-define-properties — cases

### case: different targets

```js
Object.defineProperty(t, "a", d);
Object.defineProperty(u, "b", d);
```

```expect
(none)
```

### case: not adjacent

```js
Object.defineProperty(t, "a", d);
x();
Object.defineProperty(t, "b", d);
```

```expect
(none)
```

### case: an integer after a string: no fix

```js
Object.defineProperty(t, "a", d1);
Object.defineProperty(t, 0, d2);
```

```expect
1:8 js/prefer-object-define-properties
```
