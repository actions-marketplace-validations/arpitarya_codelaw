# js/composite-assertions — cases

### case: expect

```js
expect(a && b).toBeTruthy();
```

```expect
1:1 js/composite-assertions
```

```fixed
expect(a).toBeTruthy();
expect(b).toBeTruthy();
```

### case: a single condition stays

```js
assert.ok(user);
```

```expect
(none)
```
