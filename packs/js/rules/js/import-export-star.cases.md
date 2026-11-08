# js/import-export-star — cases

### case: a named export wins nothing against a star of the same name

```js files
// src/a.js
export const x = 1;
// src/index.js
export * from "./a";
export const x = 2;
```

```expect
src/index.js:1:1 js/import-export-star
src/index.js:2:1 js/import-export-star
```

### case: default is not passed on

```js files
// src/a.js
export default 1;
export const a = 1;
// src/b.js
export default 2;
export const b = 2;
// src/index.js
export * from "./a";
export * from "./b";
```

```expect
(none)
```

### case: a star of a module with no named export

```js files
// src/a.js
export default 1;
// src/index.js
export * from "./a";
```

```expect
src/index.js:1:1 js/import-export-star
```

### case: the same names through a chain

```js files
// src/a.js
export const x = 1;
// src/mid.js
export * from "./a";
// src/index.js
export * from "./mid";
export * from "./a";
```

```expect
src/index.js:1:1 js/import-export-star
src/index.js:2:1 js/import-export-star
```

### case: a namespace re-export does not conflict

```js files
// src/a.js
export const x = 1;
// src/b.js
export const x = 2;
// src/index.js
export * as a from "./a";
export * as b from "./b";
```

```expect
(none)
```
