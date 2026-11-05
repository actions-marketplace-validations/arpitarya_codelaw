# ts/no-non-null-assertion — cases

### case: a chained member access becomes optional

```ts
const n = a.b!.c.d;
```

```expect
1:11 ts/no-non-null-assertion
```

```fixed
const n = a.b?.c.d;
```

### case: an assertion that is passed on carries no edit

```ts
run(user!);
```

```expect
1:5 ts/no-non-null-assertion
```
