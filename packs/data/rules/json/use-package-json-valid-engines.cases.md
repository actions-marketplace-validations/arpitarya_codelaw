# json/use-package-json-valid-engines — cases

### case: a range with alternatives

```json
{ "engines": { "node": "^20.19.0 || >=22.12.0", "npm": ">=10" } }
```

```expect
(none)
```

### case: a number

```json
{ "engines": { "node": 22 } }
```

```expect
1:3 json/use-package-json-valid-engines
```

### case: an array

```json
{ "engines": ["node"] }
```

```expect
1:3 json/use-package-json-valid-engines
```
