# md/no-empty-links — cases

### case: an angle-bracket empty destination is unwrapped

````md
Go to [home](<>) now.
````

```expect
1:7 md/no-empty-links
```

````fixed
Go to home now.
````

### case: a real destination

````md
Go to [home](/) now.
````

```expect
(none)
```

### case: an anchor and a bracketed path are destinations (W-139 corpus)

````md
See [the install steps](#install) and [the notes](<release notes.md>).
````

```expect
(none)
```
