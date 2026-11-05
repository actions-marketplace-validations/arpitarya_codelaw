---
id: sec-js/no-incomplete-url-check
title: "No URL checked with a substring test"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the host literal is in the code, and the reviewer picks exact host or host with its subdomains (a prefix test becomes an origin comparison either way)"
stability: preview
security: { cwe: [20], owasp: ["A01:2025"], severity: high }
since: 0.3.0
---

A URL or host is not checked with `includes`, `indexOf`, a bare `startsWith`/`endsWith` or an unanchored regular expression; it is parsed with `URL` and its `hostname` or `origin` compared.

**Why:** `url.includes("example.com")` is true for `https://evil.com/?example.com` and `https://example.com.evil.com`; the same holds for the other checks, so an attacker passes the test with a URL of their own (CWE-20).

```codelaw
match:
  any:
    - all:
        - pattern: $U.includes($H)
        - pattern: $W1
        - regex: 'includes\(\s*["''`][A-Za-z0-9.-]+\.(com|org|net|io|dev|app|co|edu|gov|ai|cloud|xyz|info|me|us|uk|de|in)["''`]\s*\)$'
    - all:
        - pattern: $U.startsWith($H)
        - pattern: $W2
        - regex: 'startsWith\(\s*["'']https?://[A-Za-z0-9.-]+["'']\s*\)$'
    - all:
        - pattern: $U.endsWith($H)
        - pattern: $W3
        - regex: 'endsWith\(\s*["''][A-Za-z0-9][A-Za-z0-9-]*\.(com|org|net|io|dev|app|co|edu|gov|ai|cloud|xyz|info|me|us|uk|de|in)["'']\s*\)$'
    - all:
        - pattern: $U.indexOf($H) !== -1
        - pattern: $W4
        - regex: 'indexOf\(\s*["''`][A-Za-z0-9.-]+\.(com|org|net|io|dev|app|co|edu|gov|ai|cloud|xyz|info|me|us|uk|de|in)["''`]\s*\)'
    - all:
        - pattern: $U.indexOf($H) > -1
        - pattern: $W5
        - regex: 'indexOf\(\s*["''`][A-Za-z0-9.-]+\.(com|org|net|io|dev|app|co|edu|gov|ai|cloud|xyz|info|me|us|uk|de|in)["''`]\s*\)'
    - all:
        - pattern: $RE.test($U)
        - regex: '^/[^^/][^/]*\\\.(com|org|net|io|dev|app|co|edu|gov|ai|cloud|xyz|info|me|us|uk|de|in)[^$]*/[gimsuy]*\.test\('
message: "A substring test on a URL passes for hosts the author did not mean, such as `example.com.evil.com`"
fix:
  choices:
    - label: exact host
      fix:
        - replace: {target: $W1, with: 'URL.canParse($U) && new URL($U).hostname === $H'}
        - replace: {target: $W3, with: 'URL.canParse($U) && new URL($U).hostname === $H'}
        - replace: {target: $W4, with: 'URL.canParse($U) && new URL($U).hostname === $H'}
        - replace: {target: $W5, with: 'URL.canParse($U) && new URL($U).hostname === $H'}
        - replace: {target: $W2, with: 'URL.canParse($U) && new URL($U).origin === $H'}
    - label: host or subdomain
      fix:
        - replace: {target: $W1, with: '(h => h === $H || h.endsWith("." + $H))(URL.canParse($U) ? new URL($U).hostname : "")'}
        - replace: {target: $W3, with: '(h => h === $H || h.endsWith("." + $H))(URL.canParse($U) ? new URL($U).hostname : "")'}
        - replace: {target: $W4, with: '(h => h === $H || h.endsWith("." + $H))(URL.canParse($U) ? new URL($U).hostname : "")'}
        - replace: {target: $W5, with: '(h => h === $H || h.endsWith("." + $H))(URL.canParse($U) ? new URL($U).hostname : "")'}
        - replace: {target: $W2, with: 'URL.canParse($U) && new URL($U).origin === $H'}
```

## How to fix

1. Parse the value: `const u = new URL(value, base)`, and compare `u.protocol` and `u.hostname` (or `u.origin`) with exact allowed values.
2. To allow subdomains, compare `u.hostname === "example.com" || u.hostname.endsWith(".example.com")` (with the leading dot).
3. A value that fails to parse is rejected.

## Examples

### JavaScript

#### Don't

```js
if (url.includes("example.com")) {
  trust(url);
}
```

#### Fixed (exact host)

```js
if (URL.canParse(url) && new URL(url).hostname === "example.com") {
  trust(url);
}
```

#### Fixed (host or subdomain)

```js
if ((h => h === "example.com" || h.endsWith("." + "example.com"))(URL.canParse(url) ? new URL(url).hostname : "")) {
  trust(url);
}
```

#### Do

```js
if (new URL(url).hostname === "example.com") {
  trust(url);
}
```

### JSX

#### Don't

```jsx
if (url.includes("example.com")) {
  trust(url);
}
```

#### Fixed (exact host)

```jsx
if (URL.canParse(url) && new URL(url).hostname === "example.com") {
  trust(url);
}
```

#### Fixed (host or subdomain)

```jsx
if ((h => h === "example.com" || h.endsWith("." + "example.com"))(URL.canParse(url) ? new URL(url).hostname : "")) {
  trust(url);
}
```

#### Do

```jsx
if (new URL(url).hostname === "example.com") {
  trust(url);
}
```

### TypeScript

#### Don't

```ts
if (url.includes("example.com")) {
  trust(url);
}
```

#### Fixed (exact host)

```ts
if (URL.canParse(url) && new URL(url).hostname === "example.com") {
  trust(url);
}
```

#### Fixed (host or subdomain)

```ts
if ((h => h === "example.com" || h.endsWith("." + "example.com"))(URL.canParse(url) ? new URL(url).hostname : "")) {
  trust(url);
}
```

#### Do

```ts
if (new URL(url).hostname === "example.com") {
  trust(url);
}
```

### TSX

#### Don't

```tsx
if (url.includes("example.com")) {
  trust(url);
}
```

#### Fixed (exact host)

```tsx
if (URL.canParse(url) && new URL(url).hostname === "example.com") {
  trust(url);
}
```

#### Fixed (host or subdomain)

```tsx
if ((h => h === "example.com" || h.endsWith("." + "example.com"))(URL.canParse(url) ? new URL(url).hostname : "")) {
  trust(url);
}
```

#### Do

```tsx
if (new URL(url).hostname === "example.com") {
  trust(url);
}
```
