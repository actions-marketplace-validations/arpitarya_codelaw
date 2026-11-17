# ts/no-enum — cases

### case: a const enum

```ts
const enum Mode { A = 1 }
```

```expect
1:1 ts/no-enum
```

### case: an ambient enum

```ts
declare enum Mode { A = 1 }
```

```expect
(none)
```
