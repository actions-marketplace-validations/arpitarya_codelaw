# js/no-mismatched-map-key — cases

### case: different literal under a negated guard

```js
if (!m.has("x")) m.set("y", 1);
```

```expect
1:24 js/no-mismatched-map-key
```

### case: reassigned receiver

```js
if (m.has(a)) {
  m = other;
  m.get(b);
}
```

```expect
(none)
```

### case: a set is not a map

```js
const s = new Set();
if (s.has(a)) s.delete(b);
```

```expect
(none)
```

### case: a key that calls something

```js
m.has(k()) && m.get(k());
```

```expect
(none)
```
