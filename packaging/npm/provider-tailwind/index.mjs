// @codelaw/provider-tailwind — builds the `tailwind` pack's class table from
// the Tailwind CSS installed in the project (SR-RULE-PACKS decision 19,
// SR-ARCHITECTURE decision 28a). codelaw's engine runs this file with Node:
// one JSON request on stdin, one JSON answer on stdout, then it exits.
//
// It holds no copy of Tailwind's order: every answer comes from the project's
// own `tailwindcss` (this package's peer dependency), so the project's version,
// its `@theme`, `@utility` and plugins order classes as its Tailwind does.
//
// Request:  {"protocol":1,"root":"/abs/project","options":{"stylesheet":"src/app.css"},
//            "rules":["tailwind/no-unknown-classes", …],"classes":["p-4", …]}
//           `rules`: the pack's rules that are on; canonical spellings are
//           worked out only for tailwind/enforces-canonical-classname.
// Answer:   {"protocol":1,"provider":"…","version":"…","tailwind":"4.3.3",
//            "inputs":["/abs/file", …],
//            "classes":{"p-4":{"order":"7","properties":["padding"],"variants":"","important":false,"canonical":null}, "foo":null, …}}
// or        {"protocol":1,"unavailable":"why"}   (no Tailwind, or one this provider cannot ask)

import { createRequire } from "node:module";
import { pathToFileURL } from "node:url";
import fs from "node:fs";
import path from "node:path";

const PROTOCOL = 1;
const NAME = "@codelaw/provider-tailwind";
const SELF_DIR = path.dirname(new URL(import.meta.url).pathname);

function readStdin() {
  return new Promise((resolve, reject) => {
    let data = "";
    process.stdin.setEncoding("utf8");
    process.stdin.on("data", (c) => (data += c));
    process.stdin.on("end", () => resolve(data));
    process.stdin.on("error", reject);
  });
}

function answer(obj) {
  process.stdout.write(JSON.stringify({ protocol: PROTOCOL, ...obj }) + "\n");
}

function selfVersion() {
  try {
    return JSON.parse(fs.readFileSync(path.join(SELF_DIR, "package.json"), "utf8")).version ?? "0.0.0";
  } catch {
    return "0.0.0";
  }
}

// The folder of an installed package, resolved as Node resolves it from `base`.
function packageDir(base, name) {
  const req = createRequire(path.join(base, "noop.js"));
  try {
    return path.dirname(req.resolve(`${name}/package.json`));
  } catch {
    // A package whose `exports` hides package.json: walk node_modules up.
    let dir = base;
    for (;;) {
      const cand = path.join(dir, "node_modules", ...name.split("/"));
      if (fs.existsSync(path.join(cand, "package.json"))) return cand;
      const up = path.dirname(dir);
      if (up === dir) return null;
      dir = up;
    }
  }
}

// The project's stylesheet: the option, else the first `.css` file under the
// root (sorted, `node_modules` and dot-folders skipped) that imports
// "tailwindcss", else Tailwind's own theme.css (what prettier-plugin-tailwindcss
// uses when it is given none).
function findStylesheet(root, options, twDir) {
  if (options.stylesheet) {
    const p = path.resolve(root, options.stylesheet);
    if (!fs.existsSync(p)) throw new Error(`[providers.tailwind] stylesheet: ${options.stylesheet} does not exist`);
    return p;
  }
  const found = [];
  const walk = (dir, depth) => {
    if (depth > 8 || found.length > 0) return;
    let names;
    try {
      names = fs.readdirSync(dir, { withFileTypes: true }).sort((a, b) => (a.name < b.name ? -1 : a.name > b.name ? 1 : 0));
    } catch {
      return;
    }
    for (const e of names) {
      if (e.name.startsWith(".") || e.name === "node_modules" || e.name === "dist" || e.name === "build") continue;
      const p = path.join(dir, e.name);
      if (e.isDirectory()) walk(p, depth + 1);
      else if (e.isFile() && e.name.endsWith(".css")) {
        const text = fs.readFileSync(p, "utf8");
        if (/@import\s+["']tailwindcss["']/.test(text)) {
          found.push(p);
          return;
        }
      }
      if (found.length > 0) return;
    }
  };
  walk(root, 0);
  if (found.length > 0) return found[0];
  return path.join(twDir, "theme.css");
}

function cssEntry(dir) {
  try {
    const pkg = JSON.parse(fs.readFileSync(path.join(dir, "package.json"), "utf8"));
    const dot = pkg.exports && (typeof pkg.exports === "string" ? null : pkg.exports["."]);
    const style = (dot && typeof dot === "object" && dot.style) || pkg.style;
    if (style) return path.join(dir, style);
  } catch {}
  return path.join(dir, "index.css");
}

function resolveCss(id, base) {
  if (id.startsWith(".") || path.isAbsolute(id)) return path.resolve(base, id);
  // `pkg` or `pkg/sub.css`
  const parts = id.split("/");
  const name = id.startsWith("@") ? parts.slice(0, 2).join("/") : parts[0];
  const rest = parts.slice(id.startsWith("@") ? 2 : 1).join("/");
  const dir = packageDir(base, name);
  if (!dir) throw new Error(`cannot find ${id} from ${base}`);
  if (!rest) return cssEntry(dir);
  const p = path.join(dir, rest);
  return fs.existsSync(p) ? p : p + ".css";
}

async function loadJs(id, base) {
  const req = createRequire(path.join(base, "noop.js"));
  const file = id.startsWith(".") || path.isAbsolute(id) ? path.resolve(base, id) : req.resolve(id);
  const mod = await import(pathToFileURL(file).href);
  return { file, module: mod.default ?? mod };
}

// Every CSS property a class's rules declare (`@property` registrations left
// out), custom properties left out unless they are all it sets.
function properties(ast) {
  const plain = new Set();
  const custom = new Set();
  const walk = (nodes) => {
    for (const n of nodes ?? []) {
      if (n.kind === "declaration") (n.property.startsWith("--") ? custom : plain).add(n.property);
      else if (n.kind === "at-rule" && n.name === "@property") continue;
      else walk(n.nodes);
    }
  };
  walk(ast);
  return [...(plain.size > 0 ? plain : custom)].sort();
}

async function main() {
  let req;
  try {
    req = JSON.parse(await readStdin());
  } catch (e) {
    return answer({ error: `the request is not JSON: ${e.message}` });
  }
  if (req.protocol !== PROTOCOL) return answer({ error: `${NAME} speaks protocol ${PROTOCOL}, the request ${req.protocol}` });
  const root = req.root;
  const options = req.options ?? {};
  const version = selfVersion();
  const twDir = packageDir(root, "tailwindcss");
  if (!twDir) return answer({ provider: NAME, version, unavailable: "no `tailwindcss` is installed in the project" });
  const twPkg = JSON.parse(fs.readFileSync(path.join(twDir, "package.json"), "utf8"));
  const inputs = new Set([path.join(twDir, "package.json")]);
  const tw = await import(pathToFileURL(createRequire(path.join(root, "noop.js")).resolve("tailwindcss")).href);
  const load = tw.__unstable__loadDesignSystem ?? tw.default?.__unstable__loadDesignSystem;
  if (typeof load !== "function") {
    return answer({ provider: NAME, version, tailwind: twPkg.version, unavailable: `tailwindcss ${twPkg.version} has no design-system API (v4 or later is needed)` });
  }
  const stylesheet = findStylesheet(root, options, twDir);
  inputs.add(stylesheet);
  const design = await load(fs.readFileSync(stylesheet, "utf8"), {
    base: path.dirname(stylesheet),
    async loadStylesheet(id, base) {
      const p = resolveCss(id, base);
      inputs.add(p);
      return { path: p, base: path.dirname(p), content: fs.readFileSync(p, "utf8") };
    },
    async loadModule(id, base) {
      const { file, module } = await loadJs(id, base);
      inputs.add(file);
      return { path: file, base: path.dirname(file), module };
    },
  });
  const wanted = [...new Set((req.classes ?? []).filter((c) => typeof c === "string" && c.length > 0))].sort();
  const timing = process.env.CODELAW_PROVIDER_TIMING ? (label, t0) => process.stderr.write(`${label} ${Math.round(performance.now() - t0)}ms\n`) : () => {};
  let t0 = performance.now();
  const order = new Map(design.getClassOrder(wanted));
  timing("order", t0);
  const known = wanted.filter((c) => order.get(c) !== null && order.get(c) !== undefined);
  t0 = performance.now();
  const asts = known.length > 0 ? design.candidatesToAst(known) : [];
  timing("ast", t0);
  // Canonical forms cost Tailwind seconds to index on first use: only when
  // a rule that reads them is on.
  const wantCanonical = (req.rules ?? []).includes("tailwind/enforces-canonical-classname");
  const classes = {};
  for (const c of wanted) classes[c] = null;
  t0 = performance.now();
  const canonical = new Map();
  if (wantCanonical) {
    for (const c of known) {
      try {
        const out = design.canonicalizeCandidates([c]);
        if (out.length === 1 && out[0] !== c) canonical.set(c, out[0]);
      } catch {}
    }
  }
  timing("canonical", t0);
  t0 = performance.now();
  known.forEach((c, i) => {
    let variants = "";
    let important = false;
    try {
      const parsed = design.parseCandidate(c)[0];
      if (parsed) {
        variants = [...parsed.variants].reverse().map((v) => design.printVariant(v)).join(":");
        important = !!parsed.important;
      }
    } catch {}
    classes[c] = { order: String(order.get(c)), properties: properties(asts[i]), variants, important, canonical: canonical.get(c) ?? null };
  });
  timing("rest", t0);
  answer({ provider: NAME, version, tailwind: twPkg.version, stylesheet, inputs: [...inputs].sort(), classes });
}

main().catch((e) => answer({ error: String(e && e.stack ? e.stack : e) }));
