# js/class-methods-use-this — cases

### case: this inside an arrow counts

```js
class A {
  run() {
    return () => this.x;
  }
}
```

```expect
(none)
```
