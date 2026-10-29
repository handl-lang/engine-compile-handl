---
title: Canonical ADR references
type: reference
date: 2026-10-16
---

# Canonical ADR references

ADRs that define ports, the kernel, or composition stay in the `platform`
repo. They are referenced here, never copied (ADR-0129 §2/§7).

| ADR | title | where |
|---|---|---|
| 0044 | Language as Data and Runtime Split | platform `project/adrs/` |
| 0074 | Arrow Surface Syntax v0.1 | platform `project/adrs/` |
| 0098 | One Language, Multiple Interpreters | platform `project/adrs/` |
| 0114 | Handl Compile Plugin Architecture | platform `project/adrs/` |
| 0128 | Target Repos | platform `project/adrs/` |
| 0129 | Project-docs Topology | platform `project/adrs/` |

## ADRs this repo owns

| # | title | file |
|---|---|---|
| 0142 | compile-handl target | `adrs/0142-compile-handl-target.md` *(reserved — lands with the module)* |

The examples corpus doubles as the compile-handl target's test input: every
file is surface syntax the compiler must accept.
