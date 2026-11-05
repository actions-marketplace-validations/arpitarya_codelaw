# ts/no-unused-vars — cases

### case: an initialiser that may do something is kept as a statement

```ts
const started = start();
export {};
```

```expect
1:7 ts/no-unused-vars
```

```fixed
start();
export {};
```

### case: a pure initialiser goes with its name

```ts
const limit = 10;
export {};
```

```expect
1:7 ts/no-unused-vars
```

```fixed
export {};
```

### case: a catch binding nothing reads

```ts
try {
  run();
} catch (err) {
  retry();
}
```

```expect
3:10 ts/no-unused-vars
```

```fixed
try {
  run();
} catch {
  retry();
}
```

### case: a parameter is left to the author

```ts
export function f(a: number, b: number) {
  return a;
}
```

```expect
1:30 ts/no-unused-vars
```
