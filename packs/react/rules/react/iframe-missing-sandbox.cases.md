# react/iframe-missing-sandbox — cases

### case: a valid sandbox

```jsx
const f = <iframe sandbox="allow-forms allow-popups" src={url} />;
```

```expect
```

### case: an empty sandbox

```jsx
const f = <iframe sandbox="" src={url} />;
```

```expect
```

### case: scripts without the origin

```jsx
const f = <iframe sandbox="allow-scripts" src={url} />;
```

```expect
```

### case: an unknown word

```jsx
const f = <iframe sandbox="allow-forms allow-script allow-popups" src={url} />;
```

```expect
1:11 react/iframe-missing-sandbox
```

```fixed
const f = <iframe sandbox="allow-forms allow-popups" src={url} />;
```

### case: an unknown word alone

```jsx
const f = <iframe sandbox="bogus" src={url} />;
```

```expect
1:11 react/iframe-missing-sandbox
```

```fixed
const f = <iframe sandbox="" src={url} />;
```

### case: unknown words first and last, in braces

```jsx
const f = <iframe sandbox={'x allow-forms  y'} src={url} />;
```

```expect
1:11 react/iframe-missing-sandbox
```

```fixed
const f = <iframe sandbox={'allow-forms'} src={url} />;
```

### case: a word that only starts like a valid one

```jsx
const f = <iframe sandbox="allow-forms-extra" src={url} />;
```

```expect
1:11 react/iframe-missing-sandbox
```

```fixed
const f = <iframe sandbox="" src={url} />;
```

### case: scripts and the origin together

```jsx
const f = <iframe sandbox="allow-scripts allow-same-origin" src={url} />;
```

```expect
1:11 react/iframe-missing-sandbox
```

```fixed
const f = <iframe sandbox="allow-scripts" src={url} />;
```

### case: the origin first

```jsx
const f = <iframe sandbox="allow-same-origin allow-forms allow-scripts" src={url} />;
```

```expect
1:11 react/iframe-missing-sandbox
```

```fixed
const f = <iframe sandbox="allow-forms allow-scripts" src={url} />;
```

### case: a sandbox held in an expression

```jsx
const f = <iframe sandbox={tokens} src={url} />;
```

```expect
```
