# js/prefer-identifier-import-export-specifiers — cases

### case: an import name

```js
import { 'default' as main } from "./main.js";
```

```expect
1:10 js/prefer-identifier-import-export-specifiers
```

```fixed
import { default as main } from "./main.js";
```

### case: a name that is not an identifier stays

```js
export { value as 'a-b' };
```

```expect
(none)
```

### case: the module path stays

```js
import { x } from 'x';
```

```expect
(none)
```
