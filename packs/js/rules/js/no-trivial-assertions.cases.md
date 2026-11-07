# js/no-trivial-assertions — cases

### case: node's assert

```js
assert.notStrictEqual(1, 2);
```

```expect
1:1 js/no-trivial-assertions
```

### case: a negated comparison

```js
expect(1).not.toBe(2);
```

```expect
1:1 js/no-trivial-assertions
```

### case: a computed value stays

```js
expect(add(1, 1)).toBe(2);
```

```expect
(none)
```
