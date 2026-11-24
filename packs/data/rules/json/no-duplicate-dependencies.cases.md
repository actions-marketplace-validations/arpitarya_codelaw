# json/no-duplicate-dependencies — cases

### case: peer and dev

```json
{ "peerDependencies": { "react": ">=18" }, "devDependencies": { "react": "^19.0.0" } }
```

```expect
(none)
```

### case: dependencies and optional

```json
{ "dependencies": { "fsevents": "^2" }, "optionalDependencies": { "fsevents": "^2" } }
```

```expect
1:67 json/no-duplicate-dependencies
```
