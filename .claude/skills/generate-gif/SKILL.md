---
name: generate-gif
description: Generate a LinkedIn-ready animated explainer GIF for Rust code — an algorithm or a single function — showing the code with the executing line highlighted, an animated visual of the data/state, a results panel and legend cards. Use when the user runs /generate-gif or asks for an animated GIF/visual explainer of code for LinkedIn or social media.
argument-hint: "<file | directory | fn name | pasted code> [title=\"...\"] [subtitle=\"...\"] [brand=\"...\"] [name=slug] [--mp4]"
allowed-tools: Read, Write, Edit, Glob, Grep, Bash(node:*), Bash(npm:*), Bash(npx playwright:*), Bash(ffmpeg:*), Bash(mkdir:*), Bash(git rev-parse:*)
---

# /generate-gif

Turn Rust code into an animated, phone-readable explainer GIF. Every GIF shares one
look, top to bottom:

1. **Header** — bold 66px title, one-line subtitle, optional round brand badge top-right.
2. **Stage** — the animated visual of the program state, usually with a white results
   `.panel` (title + purple counter like `2 / 6`) on the right.
3. **Code window** — macOS-style title bar (3 dots + file name + live narration pill),
   syntax-highlighted Rust, a purple bar sliding to the line being executed.
4. **Legend** — up to 3 lavender cards: number badge, mono title, one-line explanation.

Off-white background, white cards with soft borders, a single purple accent; green for
done, red for rejected. The header, code window and legend are built by
`assets/engine.js`; you only design the stage.

User request: $ARGUMENTS

`SKILL_DIR` below means the directory containing this file (in this repo:
`.claude/skills/generate-gif`). `ROOT` is `git rev-parse --show-toplevel` (or the cwd).

## 1. Parse the request

From the arguments / conversation, determine:

| Field | Source | Default |
|---|---|---|
| code | file, directory, fn name, or pasted code | ask only if nothing is given |
| title | `title=` / "assign this title …" | the algorithm/function name, Title Case |
| subtitle | `subtitle=` | one line: the core idea + a key fact (e.g. `3! = 6`, `O(log n)`) |
| brand | `brand=` | omit the badge |
| slug | `name=` | kebab-case of the title |
| mp4 | `--mp4` | off (MP4 is sharper on LinkedIn; GIF is always produced) |

For a **directory**, read the `.rs` files and pick the one function that best tells the
story (the algorithm's core, not `main`/tests/IO). Say which one you chose.

## 2. Prepare the code panel

The panel shows **real Rust** — never pseudo-code — but trimmed for a phone:

- **≤ 13 lines** keeps the 1080×1350 (4:5) canvas. Each extra line adds ~36px; hard cap ~24 lines.
- **≤ 54 characters per line.** Re-wrap signatures/arguments; never let a line overflow.
- Remove doc comments, attributes, asserts, logging, and anything not shown in the animation.
- Add 2–3 short trailing `// comments` on the key lines (they double as the story beats).
- Keep it something that would compile (same names, same logic).

Record the 1-based line number of each line you will highlight.

## 3. Choose the visual metaphor

The stage must make the *state change* obvious without reading the code. Pick one:

| Code shape | Stage |
|---|---|
| sorting | array of `.cell`s (or bars); compare = `.active`, swap = cells slide, sorted = `.done` |
| searching (binary/linear) | array cells + `.pointer`s (`lo`/`mid`/`hi`), discarded range faded |
| recursion / backtracking | slots filled by chips and/or a call-stack `.panel` |
| two pointers / sliding window | cells + two pointers + window highlight box |
| dynamic programming | grid of cells filled in order; dependencies flash `.active` |
| linked list / tree / graph | positioned nodes + SVG arrows; visited = `.done`, current = `.active` |
| hash map / counting | key → value rows filling in a `.panel` |
| string processing | one `.cell` per char, cursor moving, output string growing |
| iterator chain | a pipeline of stages; items flow left→right, filtered ones fade out |
| **plain function** (non-algorithm) | inputs as `.var`s → local variables update line by line (`.changed`) → return value lands in a result `.panel` |
| ownership / borrowing | stack & heap columns, boxes with arrows; move = arrow re-points, drop = box fades |

Always include a results/output panel (or the return value) with a counter when it makes
sense (`2 / 6`, `found at 4`). Use 🦀 (`.mascot`) as a cursor for Rust — it follows the
element currently being looked at.

Use a **small input** that still shows every interesting branch: aim for **25–60 steps and
8–18 s total**.

## 4. Write the scene

Copy `SKILL_DIR/templates/scene.html` to `ROOT/gifs/.work/<slug>/scene.html` and fill in
every TODO:

- A `#stage` div with the static markup and a scene `<style>` (layout only; use the UI kit
  classes from `assets/theme.css`: `.label .cell .chip .ghost .pill .panel .row .var .pointer .mascot`
  and their states `.active .done .bad .on .filled .changed .ghost .blocked`).
- A `trace()` function written in JS that **mirrors the Rust code line by line** and pushes a
  snapshot per interesting moment. Never hand-type the step list.
- `window.SCENE = { title, subtitle, brand?, file, code, legend, steps, setup(stage), render(step, i) }`

Step fields: `line` (1-based line to highlight, `0` hides it), `say` (≤ 34-char narration
shown in the code window's title bar), `ms` (hold time) plus whatever state `render` needs.
Timing guide: routine steps 120–300 ms, key moments 450–700 ms; the engine adds a 2.2 s hold on
the last step (`finalHoldMs`).

Rules for `render`:
- **Idempotent**: it sets the *entire* visual state from the step alone (no incremental DOM
  mutations). The renderer jumps around steps when measuring.
- Animate only via CSS transitions on `transform`, `opacity`, colors (all pre-declared in
  the theme). Move things with `style.transform = translate(...)`, never `left/top`.
  Override `--t` on `#stage` (e.g. `220ms`) when steps are short.
- Content must fit **984px** wide (1080 minus 48px margins). Keep the stage ≈ 380–420px tall
  to stay at 1350px; the canvas grows automatically (up to 1920px) if needed.

Readability on a phone: text ≥ 21px, values ≥ 32px, high contrast, one accent color
(purple), green = done/correct, red = rejected. No more than ~3 things moving at once.

## 5. Render

```bash
cd SKILL_DIR
[ -d node_modules ] || npm install --silent        # first run only
npx playwright install chromium                    # only if launch fails with "Executable doesn't exist"
command -v ffmpeg || echo "brew install ffmpeg"    # required
```

**Stills first** — check the layout before spending time on the full render:

```bash
node SKILL_DIR/scripts/render.mjs ROOT/gifs/.work/<slug>/scene.html --still 0,<mid>,last
```

It prints the canvas size and writes PNGs to `gifs/.work/<slug>/stills/`. **Read** them and
fix overlaps, overflow, clipped text, and wrapped labels. If it prints `canvas 1080x13xx+`
and you're above 1350, trim the code or the stage first.

Then the GIF:

```bash
node SKILL_DIR/scripts/render.mjs ROOT/gifs/.work/<slug>/scene.html --out ROOT/gifs/<slug>.gif [--mp4] [--fps 20]
```

If it warns the file is over 5 MB, retry with `--fps 15` or shorter `ms` values. Finally
extract a few frames to verify motion (e.g. `ffmpeg -i gifs/<slug>.gif -vf "select='not(mod(n,60))',scale=540:-1,tile=4x1" -frames:v 1 gifs/.work/<slug>/contact.png`)
and Read the contact sheet.

## 6. Report

Reply with the GIF path, dimensions, duration, size, which function was visualized, and
one sentence on the visual metaphor. All output lives under `ROOT/gifs/`, which is
git-ignored — never commit or push it.

## Checklist before reporting

- [ ] Follows the shared look: header, stage, code window, legend (section intro)
- [ ] Highlighted line always matches what the stage is doing
- [ ] The trace mirrors the real Rust logic; final state equals the real output
- [ ] 1080 wide; 1350 tall unless the content truly needs more; ≤ 1920
- [ ] No overlaps, no clipped/overflowing text at any step
- [ ] 8–18 s, loops, ≤ 5 MB
