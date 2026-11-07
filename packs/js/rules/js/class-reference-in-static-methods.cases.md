# js/class-reference-in-static-methods — cases

### case: the class's own name

```js
class Foo {
  static make() {
    return Foo.build();
  }
}
```

```expect
3:12 js/class-reference-in-static-methods
```

```fixed
class Foo {
  static make() {
    return this.build();
  }
}
```

### case: an instance method stays

```js
class Foo {
  make() {
    return Foo.build();
  }
}
```

```expect
(none)
```

### case: a nested function stays

```js
class Foo {
  static make() {
    return [1].map(function () { return Foo.build(); });
  }
}
```

```expect
(none)
```

### case: a write stays

```js
class Foo {
  static reset() {
    Foo.count = 0;
  }
}
```

```expect
(none)
```
