# js/no-empty — cases

### case: a lone empty block is removed

```js
run();
{}
done();
```

```expect
2:1 js/no-empty
```

```fixed
run();
done();
```

### case: an empty catch has no edit

```js
try {
  load();
} catch (e) {}
```

```expect
3:13 js/no-empty
```
