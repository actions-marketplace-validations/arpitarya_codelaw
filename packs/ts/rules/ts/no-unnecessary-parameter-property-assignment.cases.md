# ts/no-unnecessary-parameter-property-assignment — cases

### case: a later assignment is only reported

```ts
class A {
  constructor(private x: number) {
    x = x + 1;
    this.x = x;
  }
}
```

```expect
4:5 ts/no-unnecessary-parameter-property-assignment
```
