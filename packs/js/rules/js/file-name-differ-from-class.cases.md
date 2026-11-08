# js/file-name-differ-from-class — cases

### case: snake case matches

```js path=src/my_gadget.js
class MyGadget {}
export default MyGadget;
```

```expect
(none)
```

### case: a named export too

```js path=src/a.js
export default class B {}
export const c = 1;
```

```expect
(none)
```

### case: a const

```js path=src/a.js
const q = 1;
export default q;
```

```expect
1:1 js/file-name-differ-from-class
```
