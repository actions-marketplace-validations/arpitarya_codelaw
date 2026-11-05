# js/jsdoc-reject-any-type — cases

### case: any inside a generic

```js
/** @type {Array<any>} */
let list = [];
```

```expect
1:1 js/jsdoc-reject-any-type
```

### case: a word containing any

```js
/** @type {Company} */
let c = null;
```

```expect

```
