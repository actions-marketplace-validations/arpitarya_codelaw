# js/no-this-outside-of-class — cases

### case: an arrow in a method keeps the class's this

```js
class A {
  m() {
    return () => this.x;
  }
}
```

```expect
(none)
```

### case: a function nested in a method is reported

```js
class A {
  m() {
    function f() {
      return this;
    }
    return f;
  }
}
```

```expect
4:14 js/no-this-outside-of-class
```

### case: static blocks and field values

```js
class A {
  static {
    this.y = 1;
  }
  f = this.g;
}
```

```expect
(none)
```

### case: a typed this parameter

```ts
function f(this: Window) {
  return this.name;
}
```

```expect
(none)
```
