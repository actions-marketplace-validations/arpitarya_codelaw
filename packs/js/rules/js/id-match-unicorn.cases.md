# js/id-match-unicorn — cases

### case: silent without the option

```js
import { get_user } from "./users";
```

```expect
(none)
```

### case: a named specifier is checked by default

```js options={"pattern":"^[a-z][a-zA-Z0-9]*$"}
import { get_user } from "./users";
```

```expect
1:10 js/id-match-unicorn
```

### case: named specifiers left alone

```js options={"pattern":"^[a-z][a-zA-Z0-9]*$","checkNamedSpecifiers":false}
import { get_user } from "./users";
export { get_id } from "./ids";
const my_var = 1;
```

```expect
3:7 js/id-match-unicorn
```

### case: an alias is the local name

```js options={"pattern":"^[a-z][a-zA-Z0-9]*$"}
import { get_user as fetch_user } from "./users";
```

```expect
1:26 js/id-match-unicorn
```
