# js/no-process-global — cases

### case: a global read

```js
export const mode = process.env.NODE_ENV;
```

```expect
1:21 js/no-process-global
```

```fixed
import process from "node:process";
export const mode = process.env.NODE_ENV;
```

### case: after the last import

```js
import fs from "node:fs";
export const mode = process.env.NODE_ENV;
```

```expect
2:21 js/no-process-global
```

```fixed
import fs from "node:fs";
import process from "node:process";
export const mode = process.env.NODE_ENV;
```

### case: an imported process

```js
import process from "node:process";
export const mode = process.env.NODE_ENV;
```

```expect
(none)
```

### case: a local process

```js
export function run(process) {
  return process.exit();
}
```

```expect
(none)
```

### case: a property of that name

```js
export const x = config.process;
```

```expect
(none)
```
