# toml/no-duplicate-keys — edge cases

### case: array of tables repeats its header

```toml
[[bin]]
name = "a"

[[bin]]
name = "b"
```

```expect
(none)
```

### case: header after dotted keys

```toml
[tool]
x.y = 1

[tool.x]
z = 2
```

```expect
4:2 toml/no-duplicate-keys
```

### case: header defined twice

```toml
[a]
x = 1

[a]
y = 2
```

```expect
4:2 toml/no-duplicate-keys
```

### case: two values are two choices, neither applied unasked

```toml
[tool.pytest]
addopts = "-q"
addopts = "-ra"
```

```expect
3:1 toml/no-duplicate-keys
```
