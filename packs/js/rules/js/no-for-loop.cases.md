# js/no-for-loop — cases

### case: a step of two

```js
for (let i = 0; i < xs.length; i += 2) {
  log(xs[i]);
}
```

```expect
(none)
```

### case: writes to the array

```js
for (let i = 0; i < xs.length; i++) {
  xs[i] = 0;
}
```

```expect
(none)
```

### case: the index is used

```js
for (let i = 0; i < names.length; i++) {
  log(i, names[i]);
}
```

```expect
1:1 js/no-for-loop
```

```fixed
for (const [i, name] of names.entries()) {
  log(i, name);
}
```
