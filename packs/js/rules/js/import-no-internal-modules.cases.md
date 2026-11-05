# js/import-no-internal-modules — cases

### case: packages, scoped packages and parent folders

```js
import a from "lodash";
import b from "lodash/fp";
import c from "@scope/pkg";
import d from "@scope/pkg/sub";
import e from "../sibling";
import f from "../../a/b";
```

```expect
2:15 js/import-no-internal-modules
4:15 js/import-no-internal-modules
6:15 js/import-no-internal-modules
```
