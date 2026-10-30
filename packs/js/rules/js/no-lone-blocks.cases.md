# js/no-lone-blocks — cases

### case: a block scoping a let is allowed

```js
{
  let tmp = 1;
  use(tmp);
}
```

```expect

```

### case: an if body is not lone

```js
if (a) {
  run();
}
```

```expect

```
