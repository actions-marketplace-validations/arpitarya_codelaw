# node/no-top-level-await — cases

### case: a top-level for await is reported

```js
for await (const line of lines) {
  print(line);
}
```

```expect
1:1 node/no-top-level-await
```
