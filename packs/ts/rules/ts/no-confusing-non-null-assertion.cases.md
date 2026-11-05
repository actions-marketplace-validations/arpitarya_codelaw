# ts/no-confusing-non-null-assertion — cases

### case: the loose form takes the default choice

```ts
if (a! == b) run();
```

```expect
1:5 ts/no-confusing-non-null-assertion
```

```fixed
if (a == b) run();
```
