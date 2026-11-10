# template-app: the guide for coding agents

The template of a new EveryWare app (ui-lib's `scripts/new-app.sh` copies it): the time, a title, a number and a button that counts up, and no use case. It is one EveryWare app: a harry app on the EveryWare UI library, its own git repository
in the tree at `packages/services/display_safety/utils/template-app`. The project's guide (the
repositories, every skill, the rules for every change) is
`vendor/sdv-everyware/everyware/AGENTS.md`: read it first.

## What is where

| Path | What |
|---|---|
| `design/template-patch.js` | the screen: Library components in slots under `#template-root`, named `#template/...` |
| `design/build.sh` | builds the design: `design/template.fig` and `data/template.dcf` |
| `data/template.dcf` | the design the app loads; committed |
| `src/ui_model.rs` | the names of the design the code writes to or taps |
| `src/state.rs, actions.rs, presenter.rs` | the state, what a gesture changes, the state to the screen |
| `design/screen-rb3.png` | the screen on the RB3, for the README |

## The rules of an app

- **Library components only on the screen.** A new kind of control is a new library component
  (the `ui-component` skill): stop and ask the user, do not draw it in the patch.
- **What the app shows that is its own is the app's:** its pictures are a component set of its
  design, its type in the code implements `Component` with `ComponentKind::App` and `Slotted`.
  Nothing of an app goes into the UI library, also not under a generic name.
- **A bundle sends information; how it looks is decided here** (the presenter and the modules it
  uses), not in a bundle and not in the library.
- **The design's names are a contract** with `src/ui_model.rs`: change both in one commit.
- **Nothing hidden in the design** (a hidden layer never reaches the device): hide with
  `slot_shown`, `grid_shown` or an opacity.
- **Rules that need no screen** (a game, a calculation) are a module of their own with host
  tests.

## Commands

```bash
# the design: the Library, this app's .fig (design/, in git: what a designer opens) and data/template.dcf
packages/services/display_safety/utils/template-app/design/build.sh
# the app, for the RB3 (from the tree's top)
bash -c 'source device/rb3/sdv_media_db845c/tools/env.sh >/dev/null 2>&1; m template_app'
```

Running it on the RB3, a screenshot and a tap for a test: the `new-app` skill
(`vendor/sdv-everyware/ui-lib/skills/new-app/SKILL.md`), steps 7 and 8. After a design build give
the user the full path of the built `.fig` (the script's last line).

A change lands through its own branch, a review by another agent and the user's word
(`vendor/sdv-everyware/everyware/AGENTS.md`, "How a change lands"; `scripts/feature.sh` there).
The hooks in `scripts/git-hooks/` refuse a commit on `main`, by another author, with something
private in it or with an AI attribution (`vendor/sdv-everyware/everyware/scripts/setup-agents.sh`
installs them).

## This repository is the template

A change here changes every new app. Keep it free of a use case: no data source, no domain
words, one library component. After a change: `design/build.sh`, `m template_app`, run it on the
board, and `vendor/sdv-everyware/ui-lib/scripts/new-app.sh` once into a scratch name to see that
the copy still builds.
