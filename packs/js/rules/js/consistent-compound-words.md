---
id: js/consistent-compound-words
title: "A compound word in a name is one word, not two"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the new name may collide with one declared elsewhere, so the rename is offered for review"
stability: preview
ported_from: [eslint-plugin-unicorn/consistent-compound-words]
since: 0.5.0
---

Words English writes closed — `password`, `callback`, `timeout`, `metadata`, `viewport` — are one word in a camelCase name: `password`, not `passWord`; `isInViewport`, not `isInViewPort`. A declared variable, function or parameter whose name splits one of a fixed list of such words is renamed. The list is codelaw's own and conservative (words also used apart, like `fileName` or `setUp`, are not in it); upstream's is configurable.

**Why:** The same concept spelled two ways makes search and reading harder.

```codelaw
match:
  kind: identifier
  any:
    - inside: { kind: variable_declarator, field: name, inside: { kind: lexical_declaration, not: { inside: { kind: export_statement } } } }
    - inside: { kind: function_declaration, field: name, not: { inside: { kind: export_statement } } }
    - inside: { kind: formal_parameters }
    - inside: { kind: required_parameter, inside: { kind: formal_parameters } }
  regex: '(^|[a-z0-9])(?:[bB]ackGround|[cC]allBack|[cC]heckBox|[cC]lipBoard|[dD]ataBase|[dD]ownLoad|[fF]eedBack|[fF]rameWork|[kK]eyBoard|[kK]eyFrame|[lL]ifeCycle|[mM]etaData|[nN]ameSpace|[pP]assWord|[pP]ayLoad|[pP]laceHolder|[sS]creenShot|[sS]ideBar|[sS]ubDomain|[tT]imeOut|[tT]imeStamp|[tT]oolBar|[tT]oolTip|[uU]nSubscribe|[uU]pLoad|[vV]iewPort|[wW]ebHook|[wW]ebSite|[wW]orkFlow|[wW]orkSpace)([A-Z0-9_]|s?$|s[A-Z0-9_])'
  pattern: $N
transform:
  T1: { replace: { source: $N, replace: '([bB])ackGround', by: '${1}ackground' } }
  T2: { replace: { source: $T1, replace: '([cC])allBack', by: '${1}allback' } }
  T3: { replace: { source: $T2, replace: '([cC])heckBox', by: '${1}heckbox' } }
  T4: { replace: { source: $T3, replace: '([cC])lipBoard', by: '${1}lipboard' } }
  T5: { replace: { source: $T4, replace: '([dD])ataBase', by: '${1}atabase' } }
  T6: { replace: { source: $T5, replace: '([dD])ownLoad', by: '${1}ownload' } }
  T7: { replace: { source: $T6, replace: '([fF])eedBack', by: '${1}eedback' } }
  T8: { replace: { source: $T7, replace: '([fF])rameWork', by: '${1}ramework' } }
  T9: { replace: { source: $T8, replace: '([kK])eyBoard', by: '${1}eyboard' } }
  T10: { replace: { source: $T9, replace: '([kK])eyFrame', by: '${1}eyframe' } }
  T11: { replace: { source: $T10, replace: '([lL])ifeCycle', by: '${1}ifecycle' } }
  T12: { replace: { source: $T11, replace: '([mM])etaData', by: '${1}etadata' } }
  T13: { replace: { source: $T12, replace: '([nN])ameSpace', by: '${1}amespace' } }
  T14: { replace: { source: $T13, replace: '([pP])assWord', by: '${1}assword' } }
  T15: { replace: { source: $T14, replace: '([pP])ayLoad', by: '${1}ayload' } }
  T16: { replace: { source: $T15, replace: '([pP])laceHolder', by: '${1}laceholder' } }
  T17: { replace: { source: $T16, replace: '([sS])creenShot', by: '${1}creenshot' } }
  T18: { replace: { source: $T17, replace: '([sS])ideBar', by: '${1}idebar' } }
  T19: { replace: { source: $T18, replace: '([sS])ubDomain', by: '${1}ubdomain' } }
  T20: { replace: { source: $T19, replace: '([tT])imeOut', by: '${1}imeout' } }
  T21: { replace: { source: $T20, replace: '([tT])imeStamp', by: '${1}imestamp' } }
  T22: { replace: { source: $T21, replace: '([tT])oolBar', by: '${1}oolbar' } }
  T23: { replace: { source: $T22, replace: '([tT])oolTip', by: '${1}ooltip' } }
  T24: { replace: { source: $T23, replace: '([uU])nSubscribe', by: '${1}nsubscribe' } }
  T25: { replace: { source: $T24, replace: '([uU])pLoad', by: '${1}pload' } }
  T26: { replace: { source: $T25, replace: '([vV])iewPort', by: '${1}iewport' } }
  T27: { replace: { source: $T26, replace: '([wW])ebHook', by: '${1}ebhook' } }
  T28: { replace: { source: $T27, replace: '([wW])ebSite', by: '${1}ebsite' } }
  T29: { replace: { source: $T28, replace: '([wW])orkFlow', by: '${1}orkflow' } }
  T30: { replace: { source: $T29, replace: '([wW])orkSpace', by: '${1}orkspace' } }
message: "Write the compound word in `$N` as one word: `$T30`"
fix:
  - rename-symbol-local: { target: $N, to: $T30 }
```

## Examples

### JavaScript

#### Don't

```js
const passWord = read();
send(passWord);
```

#### Do

```js
const password = read();
send(password);
```


### JSX

#### Don't

```jsx
const passWord = read();
send(passWord);
```

#### Do

```jsx
const password = read();
send(password);
```


### TypeScript

#### Don't

```ts
const passWord = read();
send(passWord);
```

#### Do

```ts
const password = read();
send(password);
```


### TSX

#### Don't

```tsx
const passWord = read();
send(passWord);
```

#### Do

```tsx
const password = read();
send(password);
```
