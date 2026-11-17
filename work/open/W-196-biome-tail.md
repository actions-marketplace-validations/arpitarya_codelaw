---
type: Handoff
name: W-196
description: "W-139's Biome rows not built when it closed, 2026-11-09: 98 rows, plus 7 group switches that are not rules."
item: W-196
filed: 2026-11-09
filed_by: "Arpit (Cowork, Opus 5.5)"
updated: 2026-11-09
updated_by: "Arpit (Cowork, Opus 5.5)"
ball: agent
---

# W-196 — Biome's remaining rows

## At a glance

| step | what we do | what it gives us | state |
|---|---|---|---|
| 1 | Class the rows for frameworks codelaw has no pack for (Qwik, Solid, React Native, Vue, Svelte, Drizzle, Turbo) | what belongs here and what belongs with W-143 or nowhere | ✅ 2026-11-15: 34 rows classed in `biome.csv` |
| 2 | Build the rest in their packs | Biome's rows done | 🔄 slice 1: 18 built, 12 linked; 34 left |
| 3 | Class Biome's 7 `*/preset` rows as not rules | the ledger honest | ✅ `skip` in `biome.csv` |

**Arpit (voice, Cowork docket), 2026-11-09:** closing W-139, *"Whatever is pending, we are going to have separate work items for those. And we close W139."* These rows were not discussed on the call; they are filed here so that closing W-139 leaves nothing without a home (SR-WORK-OPEN-QUEUE rule 11). Filed on *"file it."*

**Model:** Sonnet (Claude Code), Opus for the classing in step 1.

## Goal

Every `port` row of `work/proposals/rule-parity/biome.csv` that no rule carries in `ported_from:`, by proposed pack:

| pack | rows | ids |
|---|--:|---|
| `css` | 7 | `nursery/noInvalidPropertyInitValue`, `nursery/noSelfNestedLayer`, `nursery/noUnusedClasses`, `nursery/useLayeredStyles`, `nursery/useNamedLayer`, `style/noValueAtRule`, `suspicious/useBaseline` |
| `data` | 9 | `nursery/noJsonUnsafeValues`, `nursery/useConsistentObjectKeys`, `nursery/usePackageJsonValidCpu`, `nursery/usePackageJsonValidEngines`, `style/noTopLevelLiterals`, `suspicious/noDuplicateDependencies`, `suspicious/noEmptyObjectKeys`, `suspicious/noUntrustedLicenses`, `suspicious/useRequiredScripts` |
| `js` | 48 | `complexity/noRedundantDefaultExport`, `complexity/noUselessCatchBinding`, `complexity/useSimplifiedLogicExpression`, `correctness/noPrivateImports`, `correctness/noProcessGlobal`, `correctness/noQwikUseVisibleTask`, `correctness/noVueDataObjectDeclaration`, `correctness/noVueRefAsOperand`, `correctness/noVueReservedProps`, `correctness/noVueSetupPropsReactivityLoss`, `correctness/useJsonImportAttributes`, `correctness/useQwikMethodUsage`, `correctness/useQwikValidLexicalScope`, `nursery/noBunModules`, `nursery/noDuplicateMapKeys`, `nursery/noPlaywrightNthMethods`, `nursery/noRestrictedDependencies`, `nursery/noSvelteInternal`, `nursery/noUndeclaredClasses`, `nursery/noVueMutatingProps`, `nursery/noVueUnusedEmitDeclarations`, `nursery/noYamlBooleanKeys`, `nursery/useConsistentHeadingLevel`, `nursery/useFencedCodeLanguage`, `nursery/useReactNativeValidPlatformColors`, `nursery/useSingleTopLevelHeading`, `nursery/useSvelteKitResolve`, `nursery/useSvelteKitRuneImports`, `nursery/useTopLevelHeading`, `nursery/useVueBaseImport`, `nursery/useVueMatchingComponentFileName`, `performance/noBarrelFile`, `performance/noNamespaceImport`, `performance/noReExportAll`, `performance/useTopLevelRegex`, `security/noSecrets`, `style/noExportedImports`, `style/noShoutyConstants`, `style/noUnusedTemplateLiteral`, `style/noVueOptionsApi`, `style/useNodeAssertStrict`, `style/useVueMultiWordComponentNames`, `suspicious/noDrizzleDeleteWithoutWhere`, `suspicious/noDrizzleUpdateWithoutWhere`, `suspicious/noParametersOnlyUsedInRecursion`, `suspicious/noReactNativeDeepImports`, `suspicious/noUndeclaredEnvVars`, `suspicious/useStaticResponseMethods` |
| `react` | 28 | `correctness/noJsxNamespace`, `correctness/noReactNativeRawText`, `correctness/noSolidDestructuredProps`, `correctness/useImageSize`, `correctness/useQwikClasslist`, `correctness/useQwikLoaderLocation`, `correctness/useReactNativePlatformComponents`, `correctness/useUniqueElementIds`, `nursery/noMisplacedListElements`, `nursery/noNonScalableViewport`, `nursery/noObsoleteAttributes`, `nursery/noObsoleteTags`, `nursery/noTailwindRawColors`, `nursery/noTailwindRestyledComponents`, `nursery/noUndeclaredCustomProperties`, `nursery/noUnsafeIframeSandbox`, `nursery/useReactCompiler`, `nursery/useReactNamingConvention`, `nursery/useTailwindStaticClassStrings`, `performance/useSolidForComponent`, `style/noInlineStyles`, `style/noReactNativeLiteralColors`, `style/useComponentExportOnlyModules`, `style/useNamingConvention`, `style/useReactFunctionComponents`, `suspicious/noJsxLeakedDollar`, `suspicious/noReactForwardRef`, `suspicious/noReactSpecificProps` |
| `ts` | 6 | `nursery/noMisleadingReturnType`, `nursery/noUnsafeTypeAssertion`, `nursery/useDisposables`, `nursery/useVueTypedRef`, `style/noEnum`, `suspicious/noEvolvingTypes` |

## Definition of done

| DoD | what | state |
|---|---|---|
| 1 · class | the framework rows classed, each with a reason | ✅ |
| 2 · rules | the rest built, with cases and fix classes | 🔄 |
| 3 · ledger | the 7 `*/preset` rows classed not-a-rule | ✅ |

## State (2026-11-15, slice 1)

**Classed (41 rows, `biome.csv`):** Qwik 6, Solid 3, React Native 5, Drizzle 2 and Turborepo 1 are `out-of-scope` with the question **to a new pack?** for the decider — each framework would need its own pack and registry names (W-142); Vue 11 and Svelte 3 are `out-of-scope`, W-143; `nursery/noMisleadingReturnType`, `nursery/noUnsafeTypeAssertion`, `nursery/useDisposables` are `types`, W-105; the 7 `*/preset` rows are `skip`.

**Linked (12), built by W-139 under another name:** `performance/noBarrelFile` → `js/no-barrel-files`; `performance/noNamespaceImport` → `js/import-no-namespace`; `complexity/noUselessCatchBinding` → `js/prefer-optional-catch-binding`; `style/noUnusedTemplateLiteral` → `js/no-useless-template-literals`; `correctness/noJsxNamespace` → `react/no-namespace`; `style/useComponentExportOnlyModules` → `react/only-export-components`; `style/useReactFunctionComponents` → `react/prefer-function-component`; `nursery/noPlaywrightNthMethods` → `testing/playwright-no-nth-methods`; `nursery/useConsistentHeadingLevel`, `useFencedCodeLanguage`, `useSingleTopLevelHeading`, `useTopLevelHeading` → `md/heading-increment`, `md/fence-language`, `md/single-h1`, `md/first-line-heading`.

**Built (slice 1, 18, `preview`, in no preset):** `js/use-node-assert-strict`, `js/use-json-import-attributes`, `js/no-duplicate-map-keys`, `js/no-bun-modules`, `js/use-top-level-regex`, `js/no-re-export-all`, `js/use-static-response-methods`, `ts/no-enum`, `ts/no-evolving-types`, `react/no-inline-styles`, `react/use-unique-element-ids`, `react/no-react-forward-ref`, `react/no-jsx-leaked-dollar`, `react/no-obsolete-tags`, `react/no-obsolete-attributes`, `react/no-non-scalable-viewport`, `react/no-unsafe-iframe-sandbox`, `react/no-misplaced-list-elements`.

**Left (34):** js — `no-redundant-default-export`, `use-simplified-logic-expression`, `no-private-imports`, `no-process-global`, `no-restricted-dependencies`, `no-exported-imports`, `no-shouty-constants`, `no-parameters-only-used-in-recursion`, `no-secrets`, `no-undeclared-classes`, `no-yaml-boolean-keys`; css — 7; data — 9; react — 7 (the three shadcn Tailwind rows, `no-undeclared-custom-properties`, `use-react-compiler`, `use-react-naming-convention`, `use-naming-convention`).

## Blockers

None.

## Record affected

no SR affected (SR-RULE-PACKS if a row needs a new pack).

## Hazards

- Many rows are Biome's copy of another tool's rule already built: link with `ported_from:` rather than build twice.
- Nursery rows are unstable upstream: `preview`, in no preset.

## Reference

[W-139](../../archive/open/W-139-rule-parity-build.md) · `work/proposals/rule-parity/biome.csv`
