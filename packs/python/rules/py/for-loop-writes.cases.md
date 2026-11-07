# py/for-loop-writes — cases

### case: writing the item itself

```py
with open(path, "w") as f:
    for line in lines:
        f.write(line)
```

```expect
2:5 py/for-loop-writes
```

```fixed
with open(path, "w") as f:
    f.writelines(lines)
```
