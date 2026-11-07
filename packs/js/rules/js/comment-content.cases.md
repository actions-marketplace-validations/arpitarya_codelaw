# js/comment-content — cases

### case: two names

```js
// nodejs runs javascript on the server.
```

```expect
1:1 js/comment-content
```

```fixed
// Node.js runs JavaScript on the server.
```

### case: a domain stays

```js
// Mirrored from github.com/acme/app
```

```expect
(none)
```

### case: a package name stays

```js
// Install @types/node and typescript-eslint
```

```expect
(none)
```

### case: code in backticks stays

```js
// Run `npx typescript` first.
```

```expect
(none)
```
