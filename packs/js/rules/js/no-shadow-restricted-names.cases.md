### case: a name read in a shorthand property is renamed by hand

```js
function f(NaN) {
  return { NaN };
}
```

```expect
1:12 js/no-shadow-restricted-names
```
