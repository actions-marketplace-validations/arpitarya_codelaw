# js/no-useless-delete-check — cases

### case: an else branch keeps the check

```js
if (cache.has(key)) cache.delete(key);
else miss(key);
```

```expect
(none)
```

### case: the in form

```js
if (key in store) {
  delete store[key];
}
```

```expect
1:1 js/no-useless-delete-check
```

```fixed
delete store[key];
```

### case: more work in the branch stays

```js
if (cache.has(key)) {
  cache.delete(key);
  count--;
}
```

```expect
(none)
```
