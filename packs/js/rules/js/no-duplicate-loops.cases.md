# js/no-duplicate-loops — cases

### case: an iterator helper

```js
for (const v of list.values().filter(isOn)) {
  use(v);
}
```

```expect
(none)
```
