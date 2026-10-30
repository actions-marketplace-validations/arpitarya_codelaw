# ts/adjacent-overload-signatures — cases

### case: function overloads split

```ts
function parse(x: string): number;
function other(): void {}
function parse(x: number): number;
function parse(x: unknown) { return Number(x); }
```

```expect
3:1 ts/adjacent-overload-signatures
```

### case: a comment between overloads

```ts
type T = {
  on(a: string): void
  // the second form
  on(a: number): void
}
```

```expect
```

### case: an interface's method signatures

```ts
interface Api {
  load(id: string): void;
  name: string;
  load(ids: string[]): void;
}
```

```expect
4:3 ts/adjacent-overload-signatures
```

### case: a getter and setter pair is not an overload

```ts
class A {
  get x() { return 1; }
  other() {}
  set x(v: number) {}
}
```

```expect
```
