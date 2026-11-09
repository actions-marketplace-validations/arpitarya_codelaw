# js/name-replacements — cases

### case: several replacements are choices, not a fix

```js
const e = new Error('failed');
```

```expect
1:7 js/name-replacements
```

### case: a word inside a longer name

```js
function run(createCmd) {
  return createCmd();
}
```

```expect
1:14 js/name-replacements
```

```fixed
function run(createCommand) {
  return createCommand();
}
```

### case: a camelCase replacement matches a whole name

```js
const stdDev = 1;
```

```expect
1:7 js/name-replacements
```

```fixed
const standardDeviation = 1;
```

### case: retVal

```js
let retVal = 0;
retVal += 1;
```

```expect
1:5 js/name-replacements
```

```fixed
let returnValue = 0;
returnValue += 1;
```

### case: a class

```js
class Btn {}
new Btn();
```

```expect
1:7 js/name-replacements
```

```fixed
class Button {}
new Button();
```

### case: a shorthand property keeps its key

```js
const cb = () => {};
const handlers = { cb };
```

```expect
1:7 js/name-replacements
```

```fixed
const callback = () => {};
const handlers = { cb: callback };
```

### case: a shorthand destructured name is not checked by default

```js
const { cb } = handlers;
use(cb);
```

```expect
(none)
```

### case: an exported name is reported without a fix

```js
export const cb = () => {};
```

```expect
1:14 js/name-replacements
```

### case: export of a local name keeps the exported name

```js
const cb = () => {};
export { cb };
```

```expect
1:7 js/name-replacements
```

```fixed
const callback = () => {};
export { callback as cb };
```

### case: the new name is taken

```js
const callback = 1;
const cb = 2;
use(callback, cb);
```

```expect
2:7 js/name-replacements
```

```fixed
const callback = 1;
const callback_ = 2;
use(callback, callback_);
```

### case: a name already the replacement

```js
const error = 1;
```

```expect
(none)
```

### case: constants and the allow list

```js
const CB = 1;
const getServerSideProps = 2;
const i18n = 3;
```

```expect
(none)
```

### case: a parameter documented by JSDoc is reported without a fix

```js
/**
 * @param cb the callback
 */
function run(cb) {
  return cb();
}
```

```expect
4:14 js/name-replacements
```

### case: a default import of a package is not checked

```js
import cfg from 'config';
use(cfg);
```

```expect
(none)
```

### case: a default import of a project module is

```js
import cfg from './config';
use(cfg);
```

```expect
1:8 js/name-replacements
```

```fixed
import config from './config';
use(config);
```

### case: a shorthand import of a project module

```js
import { cb } from './x';
use(cb);
```

```expect
1:10 js/name-replacements
```

```fixed
import { cb as callback } from './x';
use(callback);
```

### case: an unrelated name

```js
const items = [];
for (const item of items) use(item);
```

```expect
(none)
```

### case: a TypeScript interface

```ts
interface Cb {
  run(): void;
}
```

```expect
1:11 js/name-replacements
```

```fixed
interface Callback {
  run(): void;
}
```
