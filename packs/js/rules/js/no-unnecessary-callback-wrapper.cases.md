# js/no-unnecessary-callback-wrapper — cases

### case: reordered or partial arguments are left alone

```js
pairs.map((a, b) => f(b, a));
list.map((x) => f(x, 1));
```

```expect

```

### case: a typed parameter is not forwarded as-is

```ts
list.map((x: string) => f(x));
```

```expect

```

### case: the function called with its own parameter

```js
list.map((f) => f(f));
```

```expect
1:10 js/no-unnecessary-callback-wrapper
```
