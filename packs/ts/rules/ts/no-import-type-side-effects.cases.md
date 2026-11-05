# ts/no-import-type-side-effects — cases

### case: a value specifier keeps the import

```ts
import { type User, load } from "./user";
```

```expect
```

### case: already import type

```ts
import type { User } from "./user";
```

```expect
```
