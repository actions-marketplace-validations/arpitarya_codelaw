# js/import-export — cases

### case: two defaults

```js
export default 1;
export { x as default };
const x = 2;
```

```expect
1:1 js/import-export
2:10 js/import-export
```
