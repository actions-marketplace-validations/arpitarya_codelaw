# js/iteration-fallback-style — cases

### case: Object.entries

```js
for (const [k, v] of Object.entries(options.config ?? {})) {
  use(k, v);
}
```

```expect
1:1 js/iteration-fallback-style
```

```fixed
if (options.config) {
for (const [k, v] of Object.entries(options.config)) {
  use(k, v);
}
}
```

### case: a call source stays

```js
for (const item of load() || []) {
  use(item);
}
```

```expect
(none)
```
