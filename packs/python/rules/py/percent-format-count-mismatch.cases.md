# Cases: py/percent-format-count-mismatch

### case: too few values are reported without a fix

```py
message = "%s is %s" % ("Ada",)
```

```expect
1:11 py/percent-format-count-mismatch
```
