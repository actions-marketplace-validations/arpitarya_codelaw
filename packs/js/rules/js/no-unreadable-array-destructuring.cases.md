### case: a pattern with a type annotation is rewritten by hand

```ts
const [, , third]: string[] = parts;
```

```expect
1:7 js/no-unreadable-array-destructuring
```

### case: a right side that would bind differently is rewritten by hand

```js
const [, , third] = a || b;
```

```expect
1:7 js/no-unreadable-array-destructuring
```
