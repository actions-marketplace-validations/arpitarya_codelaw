# js/no-map-spread — cases

### case: spreading something other than the item is fine

```js
list.map((x) => ({ ...defaults, x }));
```

```expect

```

### case: a bare copy is a copy on purpose

```js
list.map((x) => ({ ...x }));
```

```expect

```
