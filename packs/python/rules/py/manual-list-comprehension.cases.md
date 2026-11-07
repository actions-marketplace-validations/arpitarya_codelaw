# py/manual-list-comprehension — cases

### case: with an if

```py
for u in users:
    if u.active:
        out.append(u.name)
```

```expect
1:1 py/manual-list-comprehension
```

```fixed
out.extend(u.name for u in users if u.active)
```

### case: a copy is not reported

```py
for u in users:
    out.append(u)
```

```expect
(none)
```
