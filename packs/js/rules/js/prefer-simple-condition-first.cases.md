# js/prefer-simple-condition-first — cases

### case: a negated name with ||

```js
if (load() || !cached) {}
```

```expect
1:5 js/prefer-simple-condition-first
```

```fixed
if (!cached || load()) {}
```

### case: names already first stay

```js
if (enabled && isValid(form)) {}
```

```expect
(none)
```
