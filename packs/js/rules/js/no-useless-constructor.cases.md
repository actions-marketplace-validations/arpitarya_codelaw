# js/no-useless-constructor — cases

### case: other arguments to super are a real constructor

```js
class A extends B {
  constructor(a) {
    super(a, 1);
  }
}
```

```expect

```
