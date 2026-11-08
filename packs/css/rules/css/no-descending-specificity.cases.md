# Cases: css/no-descending-specificity

### case: different media contexts are not compared

```css
#nav a {
  color: red;
}

@media print {
  a {
    color: black;
  }
}
```

```expect
(none)
```

### case: pseudo-elements are compared only with themselves

```css
a::before {
  content: "";
}

a {
  color: red;
}
```

```expect
(none)
```

### case: pseudo-classes are ignored when comparing

```css
a:hover {
  color: red;
}

a {
  color: blue;
}
```

```expect
5:1 css/no-descending-specificity
```

### case: nested selectors are resolved

```css
#nav {
  a {
    color: red;
  }
}

a {
  color: blue;
}
```

```expect
7:1 css/no-descending-specificity
```

### case: adjacent rules swap

```css
a:hover {
  color: red;
}
a {
  color: blue;
}
```

```expect
4:1 css/no-descending-specificity
```

```fixed
a {
  color: blue;
}
a:hover {
  color: red;
}
```

### case: a rule between them is reported without a fix

```css
#nav a {
  color: red;
}

b {
  color: green;
}

a {
  color: blue;
}
```

```expect
9:1 css/no-descending-specificity
```

### case: a selector list is reported without a fix

```css
#nav a {
  color: red;
}

a,
b {
  color: blue;
}
```

```expect
5:1 css/no-descending-specificity
```

### case: nested rules under a selector list are not swapped when two of their selectors tie

W-169 (corpus: bootstrap's `_content.scss`): the parent's two selectors differ in specificity, so `th:not(:last-child)` after `th:first-child` is "descending" for one of them, but for the other pair the two are equal and order decides. Swapping would change that cascade, and the second pass would swap them back; the finding stays, without a fix.

```scss
.a > .t,
.a > .w .t {
  th {
    &:first-child {
      padding-left: 0;
    }

    &:not(:last-child) {
      padding-right: 1rem;
    }
  }
}
```

```expect
8:5 css/no-descending-specificity
```
