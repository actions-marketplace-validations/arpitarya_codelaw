# js/promise-catch-or-return — cases

### case: a returned chain is the caller's

```js
function f() {
  return save().then(() => 1);
}
```

```expect

```

### case: an awaited chain is the caller's

```js
async function f() {
  await save().then(() => 1);
}
```

```expect

```

### case: catch then finally is handled

```js
save().catch(report).finally(done);
```

```expect

```

### case: finally alone is not

```js
save().finally(done);
```

```expect
1:1 js/promise-catch-or-return
```
