# yaml/no-boolean-keys — cases

### case: a quoted key

```yaml
"no": Norway
```

```expect
(none)
```

### case: on in a workflow

```yaml
on:
  push: {}
```

```expect
1:1 yaml/no-boolean-keys
```

```fixed
"on":
  push: {}
```

### case: a value

```yaml
country: no
```

```expect
(none)
```
