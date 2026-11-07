# js/prefer-json-import — cases

### case: a computed path stays

```js
const data = JSON.parse(fs.readFileSync(file, "utf8"));
```

```expect
(none)
```

### case: inside a function it stays

```js
function load() { const data = JSON.parse(readFileSync("./a.json", "utf8")); }
```

```expect
(none)
```
