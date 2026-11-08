# py/for-loop-set-mutations — cases

### case: a receiver not known to be a set is not reported

```py
for u in users:
    bag.add(u)
```

```expect
(none)
```
