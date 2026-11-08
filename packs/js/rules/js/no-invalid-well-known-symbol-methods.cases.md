# js/no-invalid-well-known-symbol-methods — cases

### case: an async generator on Symbol.iterator

```js
const stream = {
  async *[Symbol.iterator]() {
    yield 1;
  },
};
```

```expect
2:3 js/no-invalid-well-known-symbol-methods
```

```fixed
const stream = {
  async *[Symbol.asyncIterator]() {
    yield 1;
  },
};
```

### case: a sync dispose stays

```js
class C {
  [Symbol.dispose]() {}
}
```

```expect
(none)
```
