# json/use-required-scripts — cases

### case: a missing script

```json path=package.json options={"scripts":["build","test"]}
{ "scripts": { "build": "tsc" } }
```

```expect
1:3 json/use-required-scripts
```

### case: no option

```json
{ "scripts": {} }
```

```expect
(none)
```

### case: no scripts at all

```json path=package.json options={"scripts":["test"]}
{ "name": "x" }
```

```expect
1:1 json/use-required-scripts
```
