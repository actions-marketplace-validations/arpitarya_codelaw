# js/filename-case — cases

### case: kebab case with a second extension

```js path=src/my-utils.test.js
export const x = 1;
```

```expect
(none)
```

### case: a folder in another case

```js path=Lib/a.js
export const x = 1;
```

```expect
1:1 js/filename-case
```

### case: an index file

```ts path=src/index.ts
export const x = 1;
```

```expect
(none)
```

### case: pascal case allowed

```js path=src/MyUtils.js options={"case":"pascalCase","checkDirectories":false}
export const x = 1;
```

```expect
(none)
```

### case: an upper-case extension

```js path=src/a.JS
export const x = 1;
```

```expect
1:1 js/filename-case
```
