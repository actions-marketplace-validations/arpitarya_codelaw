# js/no-gratuitous-expressions — cases

### case: reassigned in the branch

```js
function f(a) {
  if (a) {
    a = next();
    if (a) go();
  }
}
```

```expect
(none)
```

### case: a literal condition

```js
if (true) {
  go();
}
```

```expect
1:5 js/no-gratuitous-expressions
```

### case: negated

```js
function f(a) {
  if (!a) {
    use(!a);
  }
}
```

```expect
3:10 js/no-gratuitous-expressions
```
