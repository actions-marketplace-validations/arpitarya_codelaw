# ts/no-unsafe-declaration-merging — cases

### case: an interface with a method stays manual

```ts
interface Point {
  move(): void;
}
class Point {
  x = 0;
}
```

```expect
1:11 ts/no-unsafe-declaration-merging
```
