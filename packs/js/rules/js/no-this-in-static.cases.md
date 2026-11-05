# js/no-this-in-static — cases

### case: a regular function has its own this

```js
class A {
  static f() {
    return function () { return this; };
  }
}
```

```expect

```

### case: an instance method is fine

```js
class A {
  f() { return this; }
}
```

```expect

```
