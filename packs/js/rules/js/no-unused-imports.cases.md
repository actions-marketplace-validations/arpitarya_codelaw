# js/no-unused-imports — cases

### case: a name used only as a type

```ts
import { User } from './user';
export let u: User;
```

```expect

```

### case: a component used in JSX

```jsx
import Button from './button';
export const b = <Button />;
```

```expect

```

### case: an unused default import

```js
import fs from 'node:fs';
export const x = 1;
```

```expect
1:8 js/no-unused-imports
```

### case: an unused namespace import

```js
import * as path from 'node:path';
export const x = 1;
```

```expect
1:8 js/no-unused-imports
```
