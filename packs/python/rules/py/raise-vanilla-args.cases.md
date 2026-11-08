# py/raise-vanilla-args — cases

### case: one word, a variable, NotImplementedError and bytes are fine

```py
raise Error("Short")
raise Error("its_code_not_message")
raise Error(msg)
raise NotImplementedError("This is acceptable too")
raise Error(b"two words")
```

```expect
(none)
```

### case: with a cause, and a plain ValueError

```py
raise ValueError("bad value given") from err
```

```expect
1:7 py/raise-vanilla-args
```
