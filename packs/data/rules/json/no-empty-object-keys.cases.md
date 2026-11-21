# json/no-empty-object-keys — cases

### case: a nested empty key

```json
{ "a": { "": 3 } }
```

```expect
1:10 json/no-empty-object-keys
```

### case: a key with a space

```json
{ " ": 3 }
```

```expect
(none)
```
