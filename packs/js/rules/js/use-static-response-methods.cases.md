# js/use-static-response-methods — cases

### case: with an init

```js
const res = new Response(JSON.stringify(body), { status: 201 });
```

```expect
1:13 js/use-static-response-methods
```

```fixed
const res = Response.json(body, { status: 201 });
```

### case: a text body

```js
const res = new Response("ok");
```

```expect
(none)
```
