# js/no-session-cookies-on-static-assets — cases

### case: different apps stay

```js
api.use(session({ secret }));
web.use(express.static("public"));
```

```expect
(none)
```
