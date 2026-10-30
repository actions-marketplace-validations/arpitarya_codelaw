# Cases: ts/no-unsafe-assignment

### case: a target with no declared type, or typed unknown

```ts
declare const loose: any;
export const plain = loose;
export const open: unknown = loose;
```

```expect
(none)
```

### case: a class property with a declared type

```ts
declare const loose: any;
export class Server {
  port: number = loose;
}
```

```expect
3:3 ts/no-unsafe-assignment
```
