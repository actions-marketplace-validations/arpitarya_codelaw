# Cases: scss/no-duplicate-mixins

### case: different scopes

```scss
@mixin a {
  b: c;
}
@media print {
  @mixin a {
    b: d;
  }
}
```

```expect
(none)
```

### case: a duplicate with arguments

```scss
@mixin a($x) {
  b: $x;
}
@mixin a($x, $y) {
  b: $y;
}
```

```expect
4:8 scss/no-duplicate-mixins
```

### case: an include between the two keeps the first

```scss
@mixin shadow { box-shadow: none; }
.a { @include shadow; }
@mixin shadow { box-shadow: 0 0 1px; }
```

```expect
3:8 scss/no-duplicate-mixins
```
