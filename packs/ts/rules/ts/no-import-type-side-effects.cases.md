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

### case: one specifier

```ts
import { type User } from "./user";
```

```expect
1:1 ts/no-import-type-side-effects
```

```fixed
import type { User } from "./user";
```

### case: several specifiers

```ts
import { type User, type Group, type Role } from "./model";
```

```expect
1:1 ts/no-import-type-side-effects
```

```fixed
import type { User, Group, Role } from "./model";
```

### case: aliases and line breaks

```ts
import {
  type User as Person,
  type Group,
} from "./model";
```

```expect
1:1 ts/no-import-type-side-effects
```

```fixed
import type {
  User as Person,
  Group,
} from "./model";
```

### case: a specifier named type

```ts
import { type type, type Group } from "./model";
```

```expect
1:1 ts/no-import-type-side-effects
```

```fixed
import type { type, Group } from "./model";
```

### case: a default import beside the types

```ts
import User, { type Group } from "./model";
```

```expect
```

### case: a namespace import is not a named list

```ts
import * as model from "./model";
```

```expect
```

### case: a value specifier among the types

```ts
import { type User, type Group, load } from "./model";
```

```expect
```
