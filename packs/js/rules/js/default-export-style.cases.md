# js/default-export-style — cases

### case: a class

```js
class Store {}
export default Store;
```

```expect
2:1 js/default-export-style
```

```fixed
export default class Store {}
```

### case: an arrow function stays

```js
const main = () => {};
export default main;
```

```expect
(none)
```

### case: an inline export stays

```js
export default function main() {}
```

```expect
(none)
```
