# md/fence-language — cases

### case: the default choice names the block text

````md
```
plain output
```
````

```expect
1:1 md/fence-language
```

````fixed
```text
plain output
```
````

### case: a named block

````md
```sh
ls
```
````

```expect
(none)
```
