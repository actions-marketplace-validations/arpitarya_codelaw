# ts/class-literal-property-style — cases

### case: a getter with a setter is left alone

```ts
class A {
  get mode() { return "a"; }
  set mode(v: string) {}
}
```

```expect
```

### case: a getter that computes

```ts
class A {
  get mode() { return this.m; }
}
```

```expect
```

### case: a template literal with a substitution

```ts
class A {
  get mode() { return `a${b}`; }
}
```

```expect
```
