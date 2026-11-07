# js/consistent-boolean-name — cases

### case: a Boolean() call

```js
let ready = Boolean(value);
use(ready);
```

```expect
1:5 js/consistent-boolean-name
```

```fixed
let isReady = Boolean(value);
use(isReady);
```

### case: a prefixed name stays

```js
const hasName = name !== "";
```

```expect
(none)
```

### case: a word that only starts like a prefix is reported

```js
const island = x > 1;
```

```expect
1:7 js/consistent-boolean-name
```

```fixed
const isIsland = x > 1;
```

### case: an exported name stays

```js
export const enabled = true;
```

```expect
(none)
```

### case: a number stays

```js
const count = items.length;
```

```expect
(none)
```
