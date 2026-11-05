# js/import-dynamic-import-chunkname — cases

### case: a named chunk and a computed path are left alone

```js
import(/* webpackChunkName: "home" */ "./home");
import(`./pages/${name}`);
import(path);
```

```expect

```
