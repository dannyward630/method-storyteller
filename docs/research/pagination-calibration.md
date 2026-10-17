# Pagination calibration (M2)

Status: October 2026. These rules were measured from Final Draft's own output: the PDFs it
exported and the scene pages it records in its `.fdx` files (`SceneProperties@Page`). The source
was the maintainer's private corpus, which stays off the repository. Only aggregate measurements
appear here. No script text was used, and no Final Draft code or fonts were inspected.

Ground truth is 15 FDX+PDF pairs in which Final Draft's recorded scene pages match its PDF on
every scene (1,092 scenes). `cargo xtask corpus` measures against them (see `xtask/src/corpus.rs`).

## Results

| Stage | Pairs: scenes on Final Draft's page | 108 real files | Pairs exact |
| --- | --- | --- | --- |
| M1 baseline (55 lines, 1 line before headings) | 32.3% | 23.8% | 0/15 |
| 54 lines, 2 lines before headings and shots, FDX settings read | 51.3% | 56.6% | 0/15 |
| Characters per line from a 7.04 pt advance, breaks after hyphens | 70.0% | 74.7% | 0/15 |
| (MORE) and continued cues outside the body | 80.7% | 80.8% | 6/15 |
| Parenthetical may end a split; hanging dashes | 82.7% | 81.2% | 8/15 |
| Per-paragraph overrides (indents, space before) | 98.4% | 83.7% | 10/15 |
| Leftover lines counted as laid out; wider first parenthetical line; dialogue split pieces | 99.8% | 82.8% | 13/15 |
| Dual-dialogue wrapper paragraphs no longer import as blank lines | **100%** | **87.4%** | **15/15** |

The 108 real files include drafts saved by older Final Draft versions and with other settings. Their
remaining misses are catalogued as M2 follow-up work rather than forced to fit.

## Rules found

1. **Body:** 54 lines between 1" margins (Normal leading). The page frame comes from `PageLayout`
   (margins, paper size) when a file has one.
2. **Characters per line:** Final Draft advances about 7.04 pt per character, so a 6.00" action line
   holds 61 characters and a 3.50" dialogue line 35. In the PDFs, 61-character action lines
   outnumber 60-character ones, and dialogue never passes 35 except with a hanging hyphen. The
   measured advance falls between 7.00 and 7.08 pt. Our PDF draws Courier Prime at that advance,
   so lines stay inside the margins.
3. **Hyphens:** a line may break after a hyphen inside a word. A hyphen or dash at the end of a line
   may hang one column past the margin.
4. **Parentheticals:** the first line hangs 0.10" left (`FirstIndent="-0.10"`) and holds one more
   character than the lines after it.
5. **(MORE)** prints one row below the body, in the bottom margin (baseline 730.6 pt where the last
   body line sits at 719.6 pt). It takes no body line.
6. **The continued cue** (`NAME (CONT'D)`) at the top of the next page sits one row above the body.
   It takes no body line either.
7. **Dialogue splits:**
   - The top must hold at least 2 lines, counting parentheticals.
   - A parenthetical may be the last line before (MORE). This contradicts the old spec rule.
   - Splitting inside a paragraph needs 2 of its lines on top, and 2 lines left over counted in the
     paragraph as laid out.
   - On a tie, the later split wins.
8. **Action splits:** these happen at sentence ends. At least 2 lines must stay on top, and at least
   2 must be left over, counted in the paragraph as laid out, not re-wrapped. So a 3-line paragraph
   whose split would carry over only part of a line moves whole.
9. **Per-paragraph overrides:** paragraphs carry their own `LeftIndent`, `RightIndent`,
   `SpaceBefore` and `Alignment`, and these win over the element settings. In the pairs, 291 scene
   headings use `SpaceBefore="12"` (one blank line) instead of the template's 24.
10. **Dual dialogue:** the block is wrapped in a paragraph typed `General` with no text. It is a
    container, not a blank line.

## Open questions

- Line-spacing presets (Loose, Tight, Very Tight) and `Spacing="2"` elements: none in the pairs.
- Scene `CONTINUED` markers: off in 106 of 109 files, so they are unmeasured.
- Final Draft's page-number position (7.25" or 7.5") and header layout.
- The misses on older files, especially those saved by earlier Final Draft versions (format Version
  1–3).
