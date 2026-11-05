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

### case: a loop whose body is one awaited call becomes Promise.all

```js
async function f(ids) {
  for (const id of ids) {
    await load(id);
  }
}
```

```expect
3:5 js/no-await-in-loop
```

```fixed
async function f(ids) {
  await Promise.all(ids.map((id) => load(id)));
}
```

### case: a loop that uses the result carries no edit

```js
async function f(ids) {
  for (const id of ids) {
    const r = await load(id);
    use(r);
  }
}
```

```expect
3:15 js/no-await-in-loop
```
