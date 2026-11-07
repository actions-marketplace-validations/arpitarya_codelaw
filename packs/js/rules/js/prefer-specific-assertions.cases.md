# js/prefer-specific-assertions — cases

### case: a length

```js
expect(items.length).toBe(3);
```

```expect
1:1 js/prefer-specific-assertions
```

```fixed
expect(items).toHaveLength(3);
```

### case: a value stays

```js
expect(count).toBe(3);
```

```expect
(none)
```
