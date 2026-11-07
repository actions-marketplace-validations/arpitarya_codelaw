# yaml/no-undeclared-alias — edge cases

### case: anchor in an earlier document

```yaml
base: &base 1
---
copy: *base
```

```expect
3:7 yaml/no-undeclared-alias
```

### case: an anchor defined after its alias stays manual

```yaml
production:
  <<: *defaults
  host: example.com
defaults: &defaults
  port: 443
```

```expect
2:7 yaml/no-undeclared-alias
```
