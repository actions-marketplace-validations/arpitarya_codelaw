# js/prefer-set-has — cases

### case: one search, not repeated

```js
const xs = [1, 2];
const one = xs.includes(1);
```

```expect
(none)
```

### case: also pushed to

```js
const xs = [1];
f((v) => xs.includes(v));
xs.push(2);
```

```expect
(none)
```

### case: a typed array

```ts
const xs: string[] = ["a"];
for (const v of list) {
  if (xs.includes(v)) use(v);
}
```

```expect
1:7 js/prefer-set-has
```

```fixed
const xs: Set<string> = new Set(["a"]);
for (const v of list) {
  if (xs.has(v)) use(v);
}
```

### case: minimum items

```js options={"minimumItems":3}
const xs = [1, 2];
const f = (v) => xs.includes(v);
```

```expect
(none)
```
