# js/no-useless-re-export — cases

### case: two wildcards can conflict

```js
export * from "./a.js";
export * from "./b.js";
export { x } from "./a.js";
```

```expect
(none)
```

### case: one of two names

```js
export * from "./a.js";
export { x, y as z } from "./a.js";
```

```expect
2:10 js/no-useless-re-export
```

```fixed
export * from "./a.js";
export { y as z } from "./a.js";
```

### case: default is not covered by a wildcard

```js
export * from "./a.js";
export { default } from "./a.js";
```

```expect
(none)
```
