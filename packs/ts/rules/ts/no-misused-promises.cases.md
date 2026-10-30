# Cases: ts/no-misused-promises

### case: an async callback where its Promise is used

```ts
async function save(n: number): Promise<number> {
  return n;
}
export const all = Promise.all([1, 2].map(async (n) => save(n)));
```

```expect
(none)
```

### case: a synchronous callback

```ts
[1, 2].forEach((n) => {
  console.log(n);
});
```

```expect
(none)
```

### case: an async function assigned to a void callback type

```ts
type Listener = () => void;
export const onClick: Listener = async () => {};
```

```expect
2:34 ts/no-misused-promises
```
