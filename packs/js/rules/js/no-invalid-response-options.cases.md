# js/no-invalid-response-options — cases

### case: 304

```js
const r = new Response("cached", { status: 304 });
```

```expect
1:11 js/no-invalid-response-options
```

```fixed
const r = new Response(undefined, { status: 304 });
```

### case: null stays

```js
const r = new Response(null, { status: 205 });
```

```expect
(none)
```

### case: 200 stays

```js
const r = new Response("", { status: 200 });
```

```expect
(none)
```
