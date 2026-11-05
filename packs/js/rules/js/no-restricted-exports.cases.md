# js/no-restricted-exports — cases

### case: several specifiers carry no edit

```js
export { a, main as default };
```

```expect
1:13 js/no-restricted-exports
```

### case: a re-export carries no edit

```js
export { main as default } from './main.js';
```

```expect
1:10 js/no-restricted-exports
```
