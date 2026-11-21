# js/no-redundant-default-export — cases

### case: a named function

```js
export function run() {}
export default run;
```

```expect
2:1 js/no-redundant-default-export
```

### case: a default of something else

```js
export const foo = 42;
export default 42;
```

```expect
(none)
```

### case: an export list

```js
const foo = 1;
export { foo };
export default foo;
```

```expect
3:1 js/no-redundant-default-export
```
