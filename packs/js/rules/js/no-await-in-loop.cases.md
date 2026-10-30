# js/no-await-in-loop — cases

### case: for await is serial by design

```js
async function f(s) {
  for await (const x of s) {
    await use(x);
  }
}
```

```expect

```

### case: a nested async function is its own

```js
async function f(ids) {
  for (const id of ids) {
    jobs.push(async () => await load(id));
  }
}
```

```expect

```
