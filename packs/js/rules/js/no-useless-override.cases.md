# js/no-useless-override — cases

### case: different arguments stay

```js
class A extends B {
  greet(name) {
    return super.greet(name.trim());
  }
}
```

```expect
(none)
```
