# js/no-constructor-return — cases

### case: a primitive in a base class is dropped

```js
class A {
  constructor() {
    return 1;
  }
}
```

```expect
3:5 js/no-constructor-return
```

```fixed
class A {
  constructor() {
    return;
  }
}
```

### case: an object return carries no edit

```js
class A {
  constructor() {
    return {};
  }
}
```

```expect
3:5 js/no-constructor-return
```
