# js/no-top-level-side-effects — cases

### case: no runtime export

```js
globalThis.app = 1;
```

```expect
(none)
```

### case: Object.assign on a global

```js
export const x = 0;
Object.assign(globalThis, { x });
```

```expect
2:1 js/no-top-level-side-effects
```

### case: a local object

```js
export const o = {};
Object.assign(o, { t: 1 });
```

```expect
(none)
```

### case: a shadowing local

```js
const globalThis = {};
export default 1;
globalThis.a = 1;
```

```expect
(none)
```
