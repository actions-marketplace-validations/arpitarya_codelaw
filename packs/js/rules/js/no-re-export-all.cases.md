# js/no-re-export-all — cases

### case: a namespace re-export

```js
export * as helpers from "./helpers";
```

```expect
(none)
```

### case: a wildcard

```js
export * from './helpers';
```

```expect
1:1 js/no-re-export-all
```
