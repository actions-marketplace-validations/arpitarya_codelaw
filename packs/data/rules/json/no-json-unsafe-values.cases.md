# json/no-json-unsafe-values — cases

### case: infinity

```json
[1e999]
```

```expect
1:2 json/no-json-unsafe-values
```

### case: safe numbers

```json
[9007199254740991, 1.5, 0, 1e10]
```

```expect
(none)
```

### case: a lone surrogate

```json
["\ud800"]
```

```expect
1:2 json/no-json-unsafe-values
```

### case: a surrogate pair

```json
["\ud83d\ude00"]
```

```expect
(none)
```
