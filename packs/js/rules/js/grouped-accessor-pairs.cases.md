### case: a setter above its getter moves below it

```js
class User {
  set name(v) {
    this.n = v;
  }
  save() {}
  get name() {
    return this.n;
  }
}
```

```expect
2:3 js/grouped-accessor-pairs
```

```fixed
class User {
  save() {}
  get name() {
    return this.n;
  }
  set name(v) {
    this.n = v;
  }
}
```
