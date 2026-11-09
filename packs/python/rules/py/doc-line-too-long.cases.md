# py/doc-line-too-long — cases

### case: a trailing comment is code's

```py options={"max_doc_length":40}
x = 1  # this trailing comment is longer than the limit of forty columns
```

```expect
(none)
```

### case: a docstring line

```py options={"max_doc_length":40}
def f():
    """Return one.

    This line is more than forty columns wide, so it is long.
    """
```

```expect
4:41 py/doc-line-too-long
```

### case: a long URL at the end

```py options={"max_doc_length":40}
# See https://example.com/a/very/long/path/that/goes/on/and/on/forever
```

```expect
(none)
```
