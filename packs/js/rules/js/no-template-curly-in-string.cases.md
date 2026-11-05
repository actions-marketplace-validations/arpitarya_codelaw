# js/no-template-curly-in-string — cases

### case: a template literal is fine

```js
const s = `${a}`;
```

```expect

```

### case: a string holding a backtick carries no edit

```js
const s = 'run `${cmd}`';
```

```expect
1:11 js/no-template-curly-in-string
```
