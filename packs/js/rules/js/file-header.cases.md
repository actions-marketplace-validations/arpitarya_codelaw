# js/file-header — cases

### case: silent without the option

```js
run();
```

```expect
(none)
```

### case: a missing header

```js options={"headerFormat":"// Copyright Acme\n"}
run();
```

```expect
1:1 js/file-header
```

### case: the header is there

```js options={"headerFormat":"// Copyright Acme\n"}
// Copyright Acme
run();
```

```expect
(none)
```

### case: a shebang comes first

```js options={"headerFormat":"// Copyright Acme"}
#!/usr/bin/env node
// Copyright Acme
run();
```

```expect
(none)
```

### case: a regular expression

```js options={"headerFormat":"// Copyright \\d{4} Acme","isRegularExpression":true}
// Copyright 2026 Acme
run();
```

```expect
(none)
```

### case: a regular expression that does not match

```js options={"headerFormat":"// Copyright \\d{4} Acme","isRegularExpression":true}
// Copyright Acme
run();
```

```expect
1:1 js/file-header
```
