# js/prefer-string-match-all — cases

### case: a regular expression literal

```js
while ((m = /\d+/g.exec(line)) !== null) use(m);
```

```expect
1:1 js/prefer-string-match-all
```

```fixed
for (const m of line.matchAll(/\d+/g)) use(m);
```

### case: a non-global literal stays

```js
while ((m = /\d+/.exec(line)) !== null) use(m);
```

```expect
(none)
```
