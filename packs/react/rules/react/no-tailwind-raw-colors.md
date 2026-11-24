---
id: react/no-tailwind-raw-colors
title: "A Tailwind colour utility names a token, not a palette colour"
languages: [javascript, jsx, tsx]
fix: manual
fix_reason: "which design token the colour stands for is the project's"
fix_exception: value
stability: preview
ported_from: [biome/nursery/noTailwindRawColors, eslint-plugin-shadcn/no-raw-colors]
since: 0.5.1
---

A string in a `className` or `class` attribute (including the strings inside `cn(…)`, `clsx(…)` and a conditional in it) does not use a colour utility with a raw palette colour and a shade — `bg-pink-500`, `hover:text-red-500/80`, `border-slate-200` — but a token such as `bg-primary` or `text-muted-foreground`. `white`, `black`, `transparent`, `current` and arbitrary values (`bg-[#ff00aa]`) are left alone. Behaviour of Biome's `noTailwindRawColors` on JSX; its `allowedColors` and `allowBlackAndWhite` options are not read.

**Why:** A raw palette colour bypasses the theme: dark mode, branding and contrast changes have to be made at every use instead of once in the token.

```codelaw
match:
  kind: jsx_attribute
  all:
    - has: { kind: property_identifier, regex: '^(className|class)$' }
    - has:
        stopBy: end
        any: [{ kind: string }, { kind: template_string }]
        regex: "[\\s\\\"'`:!-](bg|text|border|border-[xytrblse]|ring|ring-offset|outline|fill|stroke|from|via|to|decoration|divide|accent|caret|shadow|placeholder)-(slate|gray|zinc|neutral|stone|red|orange|amber|yellow|lime|green|emerald|teal|cyan|sky|blue|indigo|violet|purple|fuchsia|pink|rose)-(50|[1-9]00|950)(/[0-9]{1,3})?[\\s\\\"'`]"
message: "Use a theme token instead of a raw palette colour"
```

## How to fix

1. Rewrite the code as the message says.

## Examples

### JavaScript

#### Don't

```js
const b = <button className="bg-pink-500 p-2">Go</button>;
```

#### Do

```js
const b = <button className="bg-primary p-2">Go</button>;
```

### JSX

#### Don't

```jsx
const b = <button className="bg-pink-500 p-2">Go</button>;
```

#### Do

```jsx
const b = <button className="bg-primary p-2">Go</button>;
```

### TSX

#### Don't

```tsx
const b = <button className="bg-pink-500 p-2">Go</button>;
```

#### Do

```tsx
const b = <button className="bg-primary p-2">Go</button>;
```
