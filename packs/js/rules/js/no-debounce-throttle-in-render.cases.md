# js/no-debounce-throttle-in-render — cases

### case: in useMemo

```js
import { debounce } from "lodash";
import { useMemo } from "react";
function Pane() {
  const d = useMemo(() => debounce(go, 9), []);
  return d;
}
```

```expect
(none)
```

### case: a lower-case function

```js
import { debounce } from "lodash";
function pane() {
  return debounce(go, 9);
}
```

```expect
(none)
```

### case: a hook

```js
import _ from "underscore";
const useSaver = () => {
  const d = _.throttle(go, 9);
  return d;
};
```

```expect
3:15 js/no-debounce-throttle-in-render
```
