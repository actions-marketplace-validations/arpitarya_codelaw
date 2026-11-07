### case: the element name avoids one the file uses

```ts
const item = 0;
for (let i = 0; i < list.items.length; i++) {
  use(list.items[i], item);
}
```

```expect
2:1 ts/prefer-for-of
```

```fixed
const item = 0;
for (const item2 of list.items) {
  use(item2, item);
}
```
