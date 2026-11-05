// codelaw-types — the type helper (SR-TYPES decision 2). codelaw's own
// JavaScript, carried inside the engine and run with `node --input-type=module
// --eval`; it asks the project's own TypeScript (>= 7.1, `typescript/sync`)
// and nothing else. It fetches nothing (L5 decision 3).
//
// The protocol is codelaw's (decision 3): one JSON object per line on stdin,
// one answer per line on stdout, in order. The first exchange carries the
// protocol version; an engine that speaks another one refuses the helper.
//
//   -> {"op":"hello","protocol":1,"cwd":"/repo"}
//   <- {"protocol":1,"typescript":"7.1.0-beta"}
//   -> {"op":"check","file":"/repo/a.ts","text":"…","questions":[{"start":0,"end":3,"q":"any"}],"diagnostics":true}
//   <- {"answers":[true],"diagnostics":[{"start":0,"end":3,"code":2322,"category":"error","message":"…"}]}
//   -> {"op":"changed","files":["/repo/a.ts"],"created":[],"deleted":[]}
//   <- {"ok":true}
//   -> {"op":"refresh"}          a warm program re-reads what changed on disk
//   <- {"ok":true,"changed":1}
//   -> {"op":"close"}
//
// Offsets on the wire are UTF-8 bytes into `text`, as the engine counts them;
// TypeScript counts UTF-16 units, so every offset is converted here. Every
// snapshot taken is disposed: the API leaks on its Go side otherwise.

import { readFileSync, statSync } from "node:fs";
import { join } from "node:path";
import { createInterface } from "node:readline";
import { pathToFileURL } from "node:url";

const PROTOCOL = 1;
const tsDir = process.argv[process.argv.length - 1];

function send(value) {
    process.stdout.write(JSON.stringify(value) + "\n");
}

/** The module a package exports at `subpath`, as a file path. */
function exported(pkg, subpath) {
    let e = pkg.exports && pkg.exports[subpath];
    while (e && typeof e === "object") e = e.import ?? e.default ?? e.node;
    if (typeof e !== "string") throw new Error(`typescript does not export ${subpath}`);
    return join(tsDir, e);
}

let ts;
let tsVersion;
let SyntaxKind;
let loadError;
let cwd = process.cwd();
try {
    const pkg = JSON.parse(readFileSync(join(tsDir, "package.json"), "utf8"));
    tsVersion = pkg.version;
    ts = await import(pathToFileURL(exported(pkg, "./sync")).href);
    ({ SyntaxKind } = await import(pathToFileURL(exported(pkg, "./ast")).href));
} catch (e) {
    ts = undefined;
    tsVersion = tsVersion ?? null;
    loadError = String(e && e.message ? e.message : e);
}

let api;
let base;

/** The snapshot every question starts from; files open into it. */
function baseSnapshot(cwd) {
    if (!base) {
        api = new ts.API({ cwd });
        base = api.createSnapshot();
    }
    return base;
}

function replaceBase(next) {
    if (next !== base) {
        base.dispose();
        base = next;
        owners = undefined;
    }
}

/**
 * Each file of the base snapshot's programs and the one project that holds
 * it (`null` when several do), built once per snapshot: asking TypeScript
 * for a file's default project costs a round trip per file, a fifth of a
 * cold run in W-105's profiling.
 */
let owners;

function slash(path) {
    return path.replaceAll("\\", "/");
}

/** The one loaded project that holds `file` under its `tsconfig.json`'s folder, else `undefined`. */
function soleOwner(file) {
    if (!owners) {
        owners = new Map();
        for (const p of base.getProjects()) {
            if (!p.configFileName) continue;
            for (const f of p.program.getSourceFileNames()) owners.set(f, owners.has(f) ? null : p);
        }
    }
    const key = slash(file);
    const p = owners.get(key);
    if (!p) return undefined;
    const dir = slash(p.configFileName).replace(/[^/]*$/, "");
    return key.startsWith(dir) ? p : undefined;
}

/** Each file of a loaded program and its modification time, for `refresh`. */
const stamps = new Map();
const stamped = new Set();

function stamp(file) {
    try {
        return statSync(file).mtimeMs;
    } catch {
        return undefined;
    }
}

function stampProject(project) {
    if (stamped.has(project.id)) return;
    stamped.add(project.id);
    for (const f of [...project.program.getSourceFileNames(), ...project.program.getConfigFileNames()]) {
        if (!stamps.has(f)) stamps.set(f, stamp(f));
    }
}

/** The project that holds `file` in the base snapshot, opening it there. */
function projectFor(file) {
    let project = soleOwner(file) ?? base.getDefaultProjectForFile(file);
    if (!project) {
        replaceBase(base.update({ openFiles: [file], ensurePrograms: true }));
        project = base.getDefaultProjectForFile(file);
    }
    if (project) stampProject(project);
    return project;
}

function refresh() {
    if (!base) return { ok: true, changed: 0 };
    const changed = [];
    const deleted = [];
    for (const [f, was] of stamps) {
        const now = stamp(f);
        if (now === was) continue;
        stamps.set(f, now);
        (now === undefined ? deleted : changed).push(f);
    }
    if (changed.length || deleted.length) {
        replaceBase(base.update({ fileNotifications: { changed, deleted }, ensurePrograms: true }));
        // A program that changed may hold files it did not before.
        stamped.clear();
        for (const p of base.getProjects()) stampProject(p);
    }
    return { ok: true, changed: changed.length + deleted.length };
}

/** UTF-8 byte offset -> UTF-16 offset, for every byte offset in `text`. */
function byteToUtf16(text) {
    const map = new Map();
    let bytes = 0;
    let units = 0;
    for (const ch of text) {
        map.set(bytes, units);
        const cp = ch.codePointAt(0);
        bytes += cp < 0x80 ? 1 : cp < 0x800 ? 2 : cp < 0x10000 ? 3 : 4;
        units += cp >= 0x10000 ? 2 : 1;
    }
    map.set(bytes, units);
    return map;
}

function utf16ToByte(text) {
    const out = new Array(text.length + 1);
    let bytes = 0;
    let i = 0;
    for (const ch of text) {
        const cp = ch.codePointAt(0);
        const w = cp >= 0x10000 ? 2 : 1;
        for (let k = 0; k < w; k++) out[i + k] = bytes;
        i += w;
        bytes += cp < 0x80 ? 1 : cp < 0x800 ? 2 : cp < 0x10000 ? 3 : 4;
    }
    out[i] = bytes;
    return out;
}

/** The deepest node spanning exactly [start, end), else the smallest one that contains it. */
function nodeAt(sf, start, end) {
    let exact;
    let around;
    const visit = (n) => {
        const s = n.getStart(sf);
        if (s > start || n.end < end) return;
        if (s === start && n.end === end && n.kind !== SyntaxKind.ExpressionStatement) exact = n;
        if (!exact) around = n;
        n.forEachChild(visit);
    };
    sf.forEachChild(visit);
    return exact ?? around;
}

// Every type question is a generator over the API's `.gen` calls, so one
// file's questions run together through `api.batch`: each round of calls
// is one message to TypeScript, not one per call (W-105's profiling: one
// call per question was nine tenths of a cold run).

function* parts(type) {
    return type.isUnionType() ? ((yield* type.getTypes.gen()) ?? [type]) : [type];
}

function* callable(type) {
    for (const p of yield* parts(type)) if ((yield* p.getCallSignatures.gen()).length > 0) return true;
    return false;
}

/** A part whose `then` is callable, and, for `promise-like`, takes a callback first (the shape of `Promise`). */
function* thenable(checker, type, promiseShape) {
    for (const part of yield* parts(yield* checker.getApparentType.gen(type))) {
        const then = yield* part.getProperty.gen("then");
        if (!then) continue;
        for (const t of yield* parts(yield* checker.getTypeOfSymbol.gen(then))) {
            for (const sig of yield* t.getCallSignatures.gen()) {
                if (!promiseShape) return true;
                const first = (yield* sig.getParameters.gen())[0];
                if (first && (yield* callable(yield* checker.getNonNullableType.gen(yield* checker.getTypeOfSymbol.gen(first))))) return true;
            }
        }
    }
    return false;
}

/** The type a node is expected to have where it stands (an argument's parameter, an annotated target), or `undefined`. */
function* expected(checker, node) {
    try {
        return yield* checker.getContextualType.gen(node);
    } catch {
        // Not an expression: nothing is expected of it.
        return undefined;
    }
}

/** A unit type's identity: the regular type of a fresh literal, else its own. */
function unitKey(t) {
    return t.regularType || t.id;
}

/** `switch` (decision 4, `non-exhaustive`): no `default`, a discriminant made only of unit types, and one of them in no `case`. */
function* nonExhaustive(checker, node) {
    let sw = node;
    while (sw && sw.kind !== SyntaxKind.SwitchStatement) sw = sw.parent;
    if (!sw || !sw.caseBlock) return false;
    const clauses = sw.caseBlock.clauses;
    if (clauses.some((c) => c.kind === SyntaxKind.DefaultClause)) return false;
    const members = yield* parts(yield* checker.getTypeAtLocation.gen(sw.expression));
    if (!members.length || !members.every((m) => (m.flags & ts.TypeFlags.Unit) !== 0)) return false;
    const covered = new Set();
    for (const c of clauses) for (const t of yield* parts(yield* checker.getTypeAtLocation.gen(c.expression))) covered.add(unitKey(t));
    return members.some((m) => !covered.has(unitKey(m)));
}

/** `promise-to-void`: a function returning a thenable where every expected call signature returns `void`. */
function* promiseToVoid(checker, node, type) {
    let returnsThenable = false;
    for (const p of yield* parts(type)) {
        for (const s of yield* p.getCallSignatures.gen()) {
            if (yield* thenable(checker, yield* s.getReturnType.gen(), false)) returnsThenable = true;
            if (returnsThenable) break;
        }
        if (returnsThenable) break;
    }
    if (!returnsThenable) return false;
    const want = yield* expected(checker, node);
    if (!want) return false;
    const sigs = [];
    for (const p of yield* parts(want)) sigs.push(...(yield* p.getCallSignatures.gen()));
    if (sigs.length === 0) return false;
    for (const s of sigs) if (((yield* s.getReturnType.gen()).flags & ts.TypeFlags.Void) === 0) return false;
    return true;
}

/** SR-TYPES decision 4's vocabulary. A question outside it is an error. */
function* holds(checker, node, q) {
    const F = ts.TypeFlags;
    if (q === "deprecated") {
        let sym = yield* checker.getSymbolAtLocation.gen(node);
        if (sym && sym.flags & ts.SymbolFlags.Alias) sym = yield* checker.getAliasedSymbol.gen(sym);
        return !!sym && (yield* sym.getJsDocTags.gen(checker)).some((t) => t.name === "deprecated");
    }
    if (q === "non-exhaustive") return yield* nonExhaustive(checker, node);
    if (q === "typed-target") {
        const want = yield* expected(checker, node);
        return !!want && !want.isErrorType() && (want.flags & (F.AnyOrUnknown | F.TypeParameter)) === 0;
    }
    if (!QUESTIONS.has(q)) throw new Error(`unknown type question \`${q}\``);
    const type = yield* checker.getTypeAtLocation.gen(node);
    switch (q) {
        case "any":
            return (type.flags & F.Any) !== 0;
        case "never":
            return (type.flags & F.Never) !== 0;
        case "unknown":
            return (type.flags & F.Unknown) !== 0;
        case "promise-to-void":
            return yield* promiseToVoid(checker, node, type);
        case "nullable":
            return (yield* parts(type)).some((p) => (p.flags & (F.Null | F.Undefined | F.Void)) !== 0);
        case "union":
            return type.isUnionType();
        case "thenable":
            return yield* thenable(checker, type, false);
        case "promise-like":
            return yield* thenable(checker, type, true);
    }
}

/** The questions `holds` answers from the node's own type. */
const QUESTIONS = new Set(["any", "never", "unknown", "promise-to-void", "nullable", "union", "thenable", "promise-like"]);

const CATEGORY = ["warning", "error", "suggestion", "message"];

function check(req) {
    baseSnapshot(cwd);
    const file = req.file;
    let project;
    try {
        project = projectFor(file);
    } catch {
        // Not on disk yet (an editor's new buffer, a rule's example): the
        // layer below opens it.
        project = undefined;
    }
    let overlay;
    // The engine's text wins over the disk's (an editor's unsaved buffer, a
    // rule's example): the file is read from a layer in a snapshot of its own.
    let sf = project && project.program.getSourceFile(file);
    if (!sf || sf.text !== req.text) {
        overlay = base.update({ openFiles: [file], fileSystem: { kind: "layer", files: { [file]: req.text } }, ensurePrograms: true });
        project = overlay.getDefaultProjectForFile(file);
        sf = project && project.program.getSourceFile(file);
    }
    try {
        if (!project || !sf) throw new Error("no TypeScript program holds this file");
        const checker = project.checker;
        const toUnits = byteToUtf16(sf.text);
        const asks = (req.questions ?? []).map(function* ({ start, end, q }) {
            const s = toUnits.get(start);
            const e = toUnits.get(end);
            if (s === undefined || e === undefined) return null;
            const node = nodeAt(sf, s, e);
            return node ? yield* holds(checker, node, q) : null;
        });
        // One batch: the file's questions advance together, each round of
        // their calls sent as one request.
        const out = { answers: asks.length ? api.batch(...asks) : [] };
        if (req.diagnostics) {
            const toBytes = utf16ToByte(sf.text);
            const flat = (d) => (d.messageChain && d.messageChain.length ? [d.text, ...d.messageChain.map(flat)].join(" ") : d.text);
            out.diagnostics = [...project.program.getSyntacticDiagnostics(file), ...project.program.getSemanticDiagnostics(file)]
                .filter((d) => d.category === 0 || d.category === 1)
                .map((d) => ({
                    start: toBytes[Math.min(d.pos, sf.text.length)],
                    end: toBytes[Math.min(d.end, sf.text.length)],
                    code: d.code,
                    category: CATEGORY[d.category],
                    message: flat(d),
                }));
        }
        return out;
    } finally {
        if (overlay && overlay !== base) overlay.dispose();
    }
}

function changed(req) {
    if (!base) return { ok: true };
    replaceBase(
        base.update({
            fileNotifications: { changed: req.files ?? [], created: req.created ?? [], deleted: req.deleted ?? [] },
            ensurePrograms: true,
        }),
    );
    return { ok: true };
}

const lines = createInterface({ input: process.stdin, crlfDelay: Infinity });
let greeted = false;
for await (const line of lines) {
    if (!line.trim()) continue;
    let req;
    try {
        req = JSON.parse(line);
    } catch (e) {
        send({ error: `not JSON: ${e.message}` });
        continue;
    }
    if (!greeted) {
        greeted = true;
        if (req.op !== "hello" || req.protocol !== PROTOCOL) {
            send({ protocol: PROTOCOL, error: `the engine speaks protocol ${req.protocol}, this helper ${PROTOCOL}` });
            break;
        }
        if (!ts) {
            send({ protocol: PROTOCOL, typescript: tsVersion, error: `cannot load typescript/sync: ${loadError}` });
            break;
        }
        if (typeof req.cwd === "string") cwd = req.cwd;
        send({ protocol: PROTOCOL, typescript: tsVersion });
        continue;
    }
    if (req.op === "close") break;
    try {
        if (req.op === "check") send(check(req));
        else if (req.op === "changed") send(changed(req));
        else if (req.op === "refresh") send(refresh());
        else send({ error: `unknown op \`${req.op}\`` });
    } catch (e) {
        send({ error: String(e && e.message ? e.message : e) });
    }
}
if (base) base.dispose();
if (api) api.close();
// stdin may still be open (a `close` request): nothing else is left to do.
process.exit(0);
