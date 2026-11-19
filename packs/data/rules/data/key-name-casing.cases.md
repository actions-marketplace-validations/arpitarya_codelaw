# data/key-name-casing — cases

### case: silent without the option

```json
{ "max_retries": 3, "MaxDelay": 1 }
```

```expect
(none)
```

### case: a snake_case key in camelCase

```json options={"case":"camelCase"}
{ "max_retries": 3 }
```

```expect
1:4 data/key-name-casing
```

### case: nested keys

```json options={"case":"camelCase"}
{ "ok": { "inner_key": 1 }, "list": [{ "other_key": 2 }] }
```

```expect
1:12 data/key-name-casing
1:41 data/key-name-casing
```

### case: keys that are not made of words are skipped

```json options={"case":"camelCase"}
{ "$schema": "x", "@type": "y", "a b": 1, "2024": 2 }
```

```expect
(none)
```

### case: ignore

```json options={"case":"camelCase","ignore":["max_retries","/^X_/"]}
{ "max_retries": 3, "X_Y": 1, "other_one": 2 }
```

```expect
1:32 data/key-name-casing
```

### case: an acronym is words

```json options={"case":"snake_case"}
{ "userID": 1 }
```

```expect
1:4 data/key-name-casing
```

### case: a case that is not one of the five

```json options={"case":"Title\u0020Case"}
{ "a": 1, "b": 2 }
```

```expect
1:3 data/key-name-casing
```

### case: YAML block and flow keys

```yaml options={"case":"snake_case"}
maxRetries: 3
nested:
  innerKey: 1
flow: { otherKey: 2, ok_key: 3 }
```

```expect
1:1 data/key-name-casing
3:3 data/key-name-casing
4:9 data/key-name-casing
```

### case: a quoted YAML key and one with an anchor

```yaml options={"case":"snake_case"}
"maxRetries": 3
&a anchorKey: 1
```

```expect
1:2 data/key-name-casing
```

### case: TOML tables and dotted keys

```toml options={"case":"snake_case"}
[buildOptions]
maxRetries = 3
a.innerKey = 1
"quotedKey" = 2
```

```expect
1:2 data/key-name-casing
2:1 data/key-name-casing
3:3 data/key-name-casing
4:2 data/key-name-casing
```

### case: kebab-case and the others are fine as written

```toml options={"case":"kebab-case"}
[build-options]
max-retries = 3
name = "x"
```

```expect
(none)
```
