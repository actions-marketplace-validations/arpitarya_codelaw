# js/template-indent — cases

### case: already indented

```js
function run() {
  const query = sql`
    select *
    from students
  `;
  return query;
}
```

```expect
(none)
```

### case: an untagged template

```js
function run() {
  const text = `
line
`;
  return text;
}
```

```expect
(none)
```

### case: text on the first line

```js
function run() {
  const q = sql`select
from t`;
  return q;
}
```

```expect
(none)
```

### case: a substitution is kept

```js
function run() {
  const q = sql`
select ${cols}
from ${table}
where id = ${id}
`;
  return q;
}
```

```expect
2:16 js/template-indent
```

```fixed
function run() {
  const q = sql`
    select ${cols}
    from ${table}
    where id = ${id}
  `;
  return q;
}
```

### case: relative indentation stays

```js
function run() {
  const markup = html`
          <div>
              <b>x</b>
          </div>
  `;
  return markup;
}
```

```expect
2:22 js/template-indent
```

```fixed
function run() {
  const markup = html`
    <div>
        <b>x</b>
    </div>
  `;
  return markup;
}
```

### case: a function argument

```js
function run() {
  return dedent(`
text
`);
}
```

```expect
2:17 js/template-indent
```

```fixed
function run() {
  return dedent(`
    text
  `);
}
```

### case: a marking comment

```js
function run() {
  return /* HTML */ `
<p>x</p>
`;
}
```

```expect
2:21 js/template-indent
```

```fixed
function run() {
  return /* HTML */ `
    <p>x</p>
  `;
}
```

### case: the indent option

```js options={"indent":4}
function run() {
  return sql`
select 1
`;
}
```

```expect
2:13 js/template-indent
```

```fixed
function run() {
  return sql`
      select 1
  `;
}
```

### case: a tab file

```js
function run() {
	const q = sql`
select 1
`;
	return q;
}
```

```expect
2:15 js/template-indent
```

```fixed
function run() {
	const q = sql`
		select 1
	`;
	return q;
}
```

### case: a tag that is not listed

```js
function run() {
  return mine`
select 1
`;
}
```

```expect
(none)
```

### case: a snapshot

```js
it('x', () => {
  expect(value).toMatchInlineSnapshot(`
foo
`);
});
```

```expect
2:39 js/template-indent
```

```fixed
it('x', () => {
  expect(value).toMatchInlineSnapshot(`
    foo
  `);
});
```
