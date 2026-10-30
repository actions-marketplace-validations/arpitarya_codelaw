# Cases: ts/no-unsafe-argument

### case: an any argument for an unknown or generic parameter

```ts
declare const loose: any;
function show(value: unknown): void {}
function same<T>(value: T): T {
  return value;
}
show(loose);
same(loose);
```

```expect
(none)
```

### case: a call typed any passed on is reported once

```ts
declare const make: any;
function double(n: number): number {
  return n * 2;
}
double(make(1));
```

```expect
5:8 ts/no-unsafe-argument
```

### case: the second of two arguments

```ts
declare const loose: any;
function pair(a: string, b: number): void {}
pair("a", loose);
```

```expect
3:11 ts/no-unsafe-argument
```
