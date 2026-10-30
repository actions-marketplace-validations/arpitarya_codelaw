# ts/no-extraneous-class — cases

### case: a component with instance members

```ts
@Component({ tag: "x" })
export class Button {
  @Prop() size = 1;
  render() {
    return null;
  }
}
```

```expect
```

### case: an exported class with instance methods

```ts
export class A {
  private x = 1;
  get y() { return this.x; }
}
```

```expect
```
