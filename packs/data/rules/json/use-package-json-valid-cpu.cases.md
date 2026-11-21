# json/use-package-json-valid-cpu — cases

### case: a negation

```json
{ "cpu": ["!arm", "x64"] }
```

```expect
(none)
```

### case: a wrong name

```json
{ "cpu": ["amd64"] }
```

```expect
1:11 json/use-package-json-valid-cpu
```
