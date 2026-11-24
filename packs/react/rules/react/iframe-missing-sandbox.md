---
id: react/iframe-missing-sandbox
title: "An <iframe> has a sandbox"
languages: [javascript, jsx, tsx]
fix: suggest
fix_reason: "an empty `sandbox` blocks scripts, forms and same-origin access in the frame, which a page that needs them must loosen by name"
stability: preview
ported_from: [eslint-plugin-react/iframe-missing-sandbox]
since: 0.5.0
---

Every `<iframe>` has a `sandbox` attribute. A `sandbox` written as a string that holds a word the HTML standard does not define (`allow-script`) is reported, and the suggestion drops that word; a `sandbox` that gives both `allow-scripts` and `allow-same-origin` is reported too, and the suggestion drops `allow-same-origin`, since together they let the framed page remove its own sandbox. A `sandbox` whose value is an expression is left alone.

**Why:** An unsandboxed frame runs its scripts with the full rights the browser gives its origin; a misspelt word is ignored by the browser, so the frame is more open than the author meant.

```codelaw
utils:
  unknown-token:
    kind: string
    not: { regex: '^["''](\s*(allow-downloads-without-user-activation|allow-downloads|allow-forms|allow-modals|allow-orientation-lock|allow-pointer-lock|allow-popups-to-escape-sandbox|allow-popups|allow-presentation|allow-same-origin|allow-scripts|allow-storage-access-by-user-activation|allow-top-navigation-by-user-activation|allow-top-navigation-to-custom-protocols|allow-top-navigation)(\s+(allow-downloads-without-user-activation|allow-downloads|allow-forms|allow-modals|allow-orientation-lock|allow-pointer-lock|allow-popups-to-escape-sandbox|allow-popups|allow-presentation|allow-same-origin|allow-scripts|allow-storage-access-by-user-activation|allow-top-navigation-by-user-activation|allow-top-navigation-to-custom-protocols|allow-top-navigation))*)?\s*["'']$' }
  scripts-and-origin:
    kind: string
    not: { matches: unknown-token }
    all:
      - regex: '[\s"'']allow-scripts[\s"'']'
      - regex: '[\s"'']allow-same-origin[\s"'']'
match:
  any: [{ kind: jsx_opening_element }, { kind: jsx_self_closing_element }]
  all:
    - has: { field: name, regex: '^iframe$' }
    - not: { has: { kind: jsx_expression, has: { kind: spread_element } } }
    - any:
        - has: { field: name, pattern: $N }
          not: { has: { kind: jsx_attribute, regex: '^sandbox\b' } }
        - has:
            kind: jsx_attribute
            all:
              - has: { nthChild: 1, regex: '^sandbox$' }
              - has:
                  any:
                    - { kind: string, pattern: $U, matches: unknown-token }
                    - { kind: jsx_expression, has: { kind: string, pattern: $U, matches: unknown-token } }
        - has:
            kind: jsx_attribute
            all:
              - has: { nthChild: 1, regex: '^sandbox$' }
              - has:
                  any:
                    - { kind: string, pattern: $S, matches: scripts-and-origin }
                    - { kind: jsx_expression, has: { kind: string, pattern: $S, matches: scripts-and-origin } }
message: "Give this `<iframe>` a valid, narrow `sandbox`"
fix:
  - insert-after: { target: $N, text: ' sandbox=""' }
  - edit-text: { target: $U, pattern: '(["''\s])(allow-downloads-without-user-activation|allow-downloads|allow-forms|allow-modals|allow-orientation-lock|allow-pointer-lock|allow-popups-to-escape-sandbox|allow-popups|allow-presentation|allow-same-origin|allow-scripts|allow-storage-access-by-user-activation|allow-top-navigation-by-user-activation|allow-top-navigation-to-custom-protocols|allow-top-navigation)(["''\s])', replace: '$1«$2»$3' }
  - edit-text: { target: $U, pattern: '(["''\s])(allow-downloads-without-user-activation|allow-downloads|allow-forms|allow-modals|allow-orientation-lock|allow-pointer-lock|allow-popups-to-escape-sandbox|allow-popups|allow-presentation|allow-same-origin|allow-scripts|allow-storage-access-by-user-activation|allow-top-navigation-by-user-activation|allow-top-navigation-to-custom-protocols|allow-top-navigation)(["''\s])', replace: '$1«$2»$3' }
  - edit-text: { target: $U, pattern: '(^|["''\s»])([^"''\s«»]+)', replace: '$1' }
  - edit-text: { target: $U, pattern: '\s+', replace: ' ' }
  - edit-text: { target: $U, pattern: '^(["''])\s+', replace: '$1' }
  - edit-text: { target: $U, pattern: '\s+(["''])$', replace: '$1' }
  - edit-text: { target: $U, pattern: '[«»]', replace: '' }
  - edit-text: { target: $S, pattern: '(["''\s])allow-same-origin(["''\s])', replace: '$1$2' }
  - edit-text: { target: $S, pattern: '\s+', replace: ' ' }
  - edit-text: { target: $S, pattern: '^(["''])\s+', replace: '$1' }
  - edit-text: { target: $S, pattern: '\s+(["''])$', replace: '$1' }
```

## Examples

### JavaScript

#### Don't

```js
const f = <iframe src={url} title="Map" />;
```

#### Do

```js
const f = <iframe sandbox="" src={url} title="Map" />;
```

### JSX

#### Don't

```jsx
const f = <iframe src={url} title="Map" />;
```

#### Do

```jsx
const f = <iframe sandbox="" src={url} title="Map" />;
```

### TSX

#### Don't

```tsx
const f = <iframe src={url} title="Map" />;
```

#### Do

```tsx
const f = <iframe sandbox="" src={url} title="Map" />;
```
