# node/handle-callback-err — cases

### case: an arrow with one err parameter is reported

```js
emitter.on("close", (err) => done());
```

```expect
1:21 node/handle-callback-err
```

### case: err in the second place is not a callback error

```js
function f(a, err) {}
```

```expect

```

### case: a typed err parameter

```ts
function f(err: Error | null, v: number) {
  use(v);
}
```

```expect
1:1 node/handle-callback-err
```
