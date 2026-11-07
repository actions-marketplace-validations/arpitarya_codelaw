# md/no-empty-link-text — cases

### case: spaces only

```md
Read [   ](https://example.com).
```

```expect
1:6 md/no-empty-link-text
```

### case: inline code counts as text

```md
Run [`make`](https://example.com).
```

```expect
(none)
```
