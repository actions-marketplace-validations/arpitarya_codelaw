# next/no-head-import-in-document — cases

### case: a single-quoted import is rewritten

```js path=pages/_document.js
import Head from 'next/head';
```

```expect
1:1 next/no-head-import-in-document
```

```fixed
import { Head } from "next/document";
```
