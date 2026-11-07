# js/no-ineffective-csp-directives — cases

### case: report-uri

```jsx
const m = <meta http-equiv="content-security-policy" content="default-src 'self'; report-uri /csp" />;
```

```expect
1:11 js/no-ineffective-csp-directives
```

### case: another meta stays

```jsx
const m = <meta name="description" content="sandbox mode" />;
```

```expect
(none)
```
