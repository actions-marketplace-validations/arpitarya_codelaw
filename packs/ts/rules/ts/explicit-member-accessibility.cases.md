# ts/explicit-member-accessibility — cases

### case: private names, declare fields and decorated fields

```ts
class A {
  #secret = 1;
  declare x: number;
  @Input() size = 1;
}
```

```expect
```

### case: a constructor and its parameter property

```ts
class A {
  constructor(readonly id: string) {}
}
```

```expect
2:3 ts/explicit-member-accessibility
2:15 ts/explicit-member-accessibility
```

```fixed
class A {
  public constructor(public readonly id: string) {}
}
```

### case: a decorated method keeps its decorator first

```ts
class A {
  @HostListener("click")
  onClick() {}
}
```

```expect
3:3 ts/explicit-member-accessibility
```

```fixed
class A {
  @HostListener("click")
  public onClick() {}
}
```
