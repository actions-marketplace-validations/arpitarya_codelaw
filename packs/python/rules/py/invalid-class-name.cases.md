# py/invalid-class-name — cases

### case: a public class is reported without a fix

```py
class user_record:
    pass
```

```expect
1:7 py/invalid-class-name
```

### case: CapWords, with or without a leading underscore, is not reported

```py
class UserRecord:
    pass


class _Private2:
    pass
```

```expect
(none)
```
