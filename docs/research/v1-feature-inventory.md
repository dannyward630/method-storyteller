# Build the production core, then prove parity

Status: research, October 2026. Clean-room: everything is described in our own words from public
sources (Final Draft's manuals, knowledge base, release notes and official tutorial videos,
public FDX files under permissive licences, other screenwriting apps' documentation) and from structural
statistics of a private corpus of real FDX files. Supporting notes are in [v1-notes/](v1-notes/).

### About the private corpus

Counts marked "corpus" come from a structural scan of the maintainer's own FDX files (and their
matching PDFs) kept on one machine. The scan recorded only settings values and counts, never script
text, titles or names. The files and the detailed notes stay off the repository on purpose.

Method Storyteller V1.0 should match Final Draft's professional core, not its whole menu. That core is about 85 features, counting those V1 only has to preserve: Final Draft's element flow and SmartType, pagination that matches Final Draft's page for page, lossless FDX round-trip, and the production toolkit (scene numbers with A-suffixes, omitted scenes, revision sets and marks, locked pages). Reports, spell check, notes and templates complete it. The Beat Board, Outline Editor, collaboration and the comfort features of FD13 can wait, but their FDX data must survive round-trip from day one. Three sources draw the same line. Final Draft's own mobile app, FD Go, ships about this subset ([FD Go](https://www.finaldraft.com/products/final-draft-iphone-ipad/)). And the maintainer's own 111 real FDX files use dual dialogue, styled text, per-paragraph format overrides, scene numbers and revisions in most projects, but never use alternates, breakdown tags or A-pages ([corpus notes][corpus]). Measured against that line, the MVP is a sound engine: deterministic pagination, Fountain, PDF, a command system and a control channel. But it fully delivers only 13 of the 85 V1 features and partly delivers 24 more. Its spec also gets the two biggest page-count drivers wrong: Final Draft uses 54 lines per page, not 55, and 2 blank lines before scene headings and shots, not 1. Its FDX importer has bugs that print hidden text as action. The maintainer's corpus contains 16 FDX+PDF pairs where Final Draft's own PDF agrees with the file on every scene's page, 973 scenes in all. With them, almost every pagination question can be settled before anyone opens a licensed copy of Final Draft 11. That session can then shrink to a short, scripted batch at the end.

## Three evidence streams agree on a production-first V1

Final Draft's documentation describes a product that has been stable at its core for years. The FD13 user guide is laid out by menu. The parts a parity-minded app must match are the File and Export set, Edit's find, replace and go-to tools, the script views, Format > Elements, Document's page layout, header and footer, mores and continueds, title page and SmartType dialogs, and the Production menu ([FD13 guide][fd13]). Release notes show that revisions, page locking and scene numbering predate FD11. Most of what FD12 and FD13 added is outlining and comfort: Track Changes, the Outline Editor, Focus Mode, Writing Stats, Typewriter, Midnight Mode, Navigator 2.0 and Structure Lines ([FD12 notes][n12], [FD13.0 notes][n13]). So **a clone aimed at the production core is aiming at a fixed target**. No Final Draft 14 was found, and the newest product is a browser-based Cloud editor ([manuals index](https://www.finaldraft.com/support/manuals-and-software/)).

The official tutorial videos add behaviour the manuals only imply. The status bar always shows what Tab and Enter will create next. Enter on an empty line opens a floating element picker with single-letter keys. Holding Cmd or Ctrl shows numbered element shortcuts that change with context ([Quick Start FD13][qs13]). Locking pages turns revision mode on automatically. Overflow then becomes 10A, and each relock deepens the lettering to AA and AAA ([Locked Pages FD12][lock12]). Hiding outline paragraphs repaginates the script; one demo went from 49 pages to 41 ([Idea to Script FD13][s123]). These details decide whether Final Draft users feel at home. Most cost little.


The maintainer's corpus decides the close calls. All 127 cloud FDX files were hydrated. That gave 111 unique real-user files in about 43 projects, 14 of them feature-length. They are all Version 1–5; none is Version 6 ([corpus notes][corpus]). Counting per project avoids letting draft families of 13–18 files dominate. On that basis, **dual dialogue (14/14 projects), bold/italic/underline (14/14), per-paragraph format overrides (13/14), scene numbers (12/14) and revision marks (9/14)** are mainstream. Deleted-text revision markers (8/14), letter-suffixed or prefixed scene numbers (6/14), omitted scenes (4/14), locked pages (3/14) and script notes (3/14) are a real minority. **Alternates, breakdown tags, A-pages, scene colours and real Beat Board use never appear** ([corpus notes][corpus]). Every settings section is remarkably uniform: 105 of 109 complete saves use the same indent family, and every file has the same Return-key flow. Final Draft's defaults can therefore be read directly from the data Final Draft writes.

## The unified V1.0 feature inventory

Each row gives a tag (**v1**, **later**, **skip**), a one-line reason and its evidence, plus its status in the current MVP (**done**, **partial**, **missing**). "v1 (preserve)" means V1 must round-trip the data through FDX untouched, but needs no UI for it. Corpus counts are "files" out of 111 real files, or "projects" out of 14 feature-length projects. Status was judged from `docs/spec.md` and a skim of `crates/` ([spec][spec], [model.rs][model], [flow.rs][flow], [layout.rs][layout], [fdx.rs][fdxrs], [paginate.rs][paginate], [commands.rs][commands]).

### Writing and element flow

| Feature | Tag | Reason | Evidence | MVP |
|---|---|---|---|---|
| Core elements: Scene Heading, Action, Character, Parenthetical, Dialogue, Transition, Shot | v1 | Used in nearly every real file | [FD13 guide][fd13] §1.17; corpus: 210,222 paragraphs | done |
| General element | v1 | FD's catch-all element; 166 paragraphs in 66 files | [corpus][corpus] | missing (imported as Action) |
| New Act / End of Act | v1 | Needed for faithful TV import; defined in 17 files, used in 2 | [FD13 guide][fd13] §1.26; [corpus][corpus] | missing (printed as Action) |
| Custom element types, paginated by their "paginate as" rule | v1 import, later UI | A file's element list is whatever its settings define (Lyrics, Onscreen Text seen) | [FDX notes][fdxnotes] Q1; [Custom Elements FD12][ce12] | missing |
| Outline elements (Outline 1–4, Summary, Note, Sequence), hidden from print | v1 | Hiding them changes page count; they are currently printed | [Outline Elements FD13][oel13]; [S17] | partial (Section/Synopsis exist but are not mapped) |
| Cast List element | later, v1 (preserve) | Multi-cam only; no TV templates in the corpus | [FD13 guide][fd13] §1.17 | missing |
| Enter/Tab flow with FD defaults, as data per element | v1 | Muscle memory is Final Draft's moat; 109/109 files share one Return table | [FD13 guide][fd13] §1.26; [corpus][corpus] | partial (three defaults differ, see spec corrections) |
| Element picker on Enter in an empty paragraph | v1 | FD's way to switch elements without shortcuts | [Quick Start FD13][qs13] | missing |
| Cmd/Ctrl+0–9 element shortcuts; modifier to convert in place | v1 | Documented core shortcuts | [FD13 guide][fd13] §1.26.3; [Quick Start FD12][qs12] | partial (1–7 only) |
| Status-bar hint of the next Tab/Enter element | v1 | Cheap discoverability, always on screen in FD | [Quick Start FD13][qs13] | missing (shows current element only) |
| Auto parentheses for parentheticals | v1 | FD types both brackets | [FD13 guide][fd13] §1.17 | done |
| Caps by element (heading, cue, transition, shot) | v1 | AllCaps in element settings of every file | [corpus][corpus] | done |
| Typing "int." or "ext." in Action makes a heading | v1 | Documented auto-behaviour, trivial | [FD13 guide][fd13] §1.26.5 | missing |
| Capitalise first word of a sentence (preference) | v1 | Documented default | [FD13 guide][fd13] §1.4 | missing |
| Automatic character (CONT'D) within a scene | v1 | On in 105/109 files; seen in every demo; changes text, not line counts | [Mores KB][kbmores]; [Typewriter FD13][tw13]; [corpus][corpus] | missing (spec defers it) |
| SmartType: six lists stored in the document, learn-as-you-type, rebuild, separators | v1 | Users add their own entries: times of day in 92/109 files, transitions 64, intros 61, extensions 57 | [FD13 guide][fd13] §1.25; [corpus][corpus] | partial (suggestions scanned from script text; no stored lists) |
| SmartType guesses the next speaker in a two-hander | v1 | Small and loved; part of the documented core | [FD13 guide][fd13] §1.25 | missing |
| Smart quotes | v1 | On in 104/111 files | [corpus][corpus] | missing |
| Macros, list continuation, custom autocorrect | later | Comfort features, FD13.x era | [FD13.2 notes][n132] | missing |
| Dictation, emoji colour rendering | skip / later | OS plumbing | [FD13 guide][fd13] §1.7 | — |

### Editing tools

| Feature | Tag | Reason | Evidence | MVP |
|---|---|---|---|---|
| Undo/redo | v1 | Baseline | [FD13 guide][fd13] §1.7 | done (500 snapshots) |
| Element-preserving copy and paste | v1 | Pasting FD text must keep element types | [FD13 guide][fd13] §1.7 | missing (no clipboard command) |
| Find and replace (case, whole word, per element), with undoable Replace All | v1 | FD's Replace All cannot be undone, an easy win | [FD13 guide][fd13] §1.7 | missing |
| Go to page, scene or bookmark; find selection; select scene | v1 | Cheap navigation the docs treat as core | [FD13 guide][fd13] §1.7 | missing |
| Rename character everywhere, undoable | v1 | FD renames cues, CONT'D cues and lists | [Navigator Characters FD13][navc13] | missing |
| Spell check as you type and in a dialog, with ignore lists stored in FDX | v1 | Expected of a professional writing app; the ignore lists are in every FDX version | [corpus][corpus] | missing |
| Smart drag-and-drop of dialogue with its cue | later | Nice, not essential | [FD13 guide][fd13] §1.7 | missing |
| Track Changes | later, v1 (preserve) | FD12 co-writing tool; production uses revisions | [Track Changes FD12][tc12] | missing |
| Thesaurus | later | Comfort | [FD13 guide][fd13] §1.12 | missing |

### Formatting and page layout

| Feature | Tag | Reason | Evidence | MVP |
|---|---|---|---|---|
| Element specs from the file (font, indents, space before, spacing, alignment) drive layout | v1 | FD13 bodies carry almost no layout, so layout must come from the settings; Fade In does this | [FDX notes][fdxnotes] Q1; [Fade In KB][fadeinkb] | missing (fixed table) |
| Per-paragraph format overrides | v1 | 3,499 paragraphs in 75 files; 13/14 projects | [corpus][corpus] | missing (dropped) |
| Bold, italic, underline runs | v1 | 14/14 projects | [corpus][corpus] | done |
| AllCaps run style | v1 | 4,830 runs plus combinations; dropping it shows lowercase where FD shows caps | [corpus][corpus] | missing |
| Strikeout | v1 | Cheap; 1/14 projects | [corpus][corpus] | missing |
| Run font, size, colour, highlight | later, v1 (preserve) | Non-default fonts in 2/14 projects | [corpus][corpus] | missing |
| 54 lines per page on Normal | v1 | FD's documented figure; Loose 49, Tight 58, Very Tight 64 | [lines-per-page KB][kblines] | partial (55) |
| Line-spacing presets | later | Users cheat with them, but no real file needs one | [lines-per-page KB][kblines] | missing |
| Space before in points; 2 lines before headings and shots | v1 | All FD screenplay templates; 105/109 files | [two-lines KB][kb2lines]; [corpus][corpus] | partial (1 line) |
| Break action and dialogue only at sentence ends | v1 (default on), later toggle | Default in 109/109 files | [sentence-break KB][kbsentence] | done |
| Keep-together rules (heading, cue, transition, dual dialogue) | v1 | The pagination core | [lines-per-page KB][kblines]; [Talentville][talentville] | done (thresholds unverified) |
| (MORE)/(CONT'D) at dialogue breaks, texts editable and read from FDX | v1 | On in 109/109 files | [Mores KB][kbmores]; [corpus][corpus] | partial (texts fixed, not imported) |
| Scene CONTINUED top and bottom with optional count | v1 (when enabled) | Off by default but on in 3 files; it changes page count | [Mores KB][kbmores]; [corpus][corpus] | missing |
| Header and footer with fields (page, revision label, date), first-page toggle, starting page | v1 | Page # in 111 files, Last Revised 35, Collated 10 | [FD13 guide][fd13] §1.23; [corpus][corpus] | partial (fixed page number) |
| US Letter | v1 | All files that state a size | [corpus][corpus] | done |
| A4 | later | 0 A4 files; FD's A4 rules undocumented; keep paper size in the model | [paper-size KB](https://kb.finaldraft.com/hc/en-us/articles/44986753080724-How-do-I-change-the-paper-size-and-layout-of-my-file-in-Final-Draft-Writer) | missing |
| Title page as free layout: aligned paragraphs, tab stops, more than one page | v1 | FD stores no fields, only layout; 109 files, mostly Left and Center | [FDX notes][fdxnotes] Q1; [Title Page FD12][tp12]; [corpus][corpus] | partial (Fountain-style fields; FDX layout lost) |
| Images on the title page or in the script | later, v1 (preserve) | No images in any real file | [corpus][corpus] | missing |
| Free Courier with per-line capacity calibrated to Courier Final Draft | v1 | FD's font is not exactly 10 cpi, so words wrap differently | [Fade In KB][fadeinkb] | partial (Courier Prime, strict 10 cpi) |
| Page view; Normal and Speed views | v1 / later | Page view is FD's default | [FD13 guide][fd13] §1.8 | done (page view) |
| Zoom; show invisibles; show or hide note markers | v1 | Low cost | [FD13 guide][fd13] §1.8 | partial (zoom) |
| Light and dark appearance | v1 | FD has Day, Night, Midnight and System | [Midnight FD13][mm13] | partial (page always light) |
| Watermark | later, v1 (preserve) | Text set in only 5 files | [corpus][corpus] | missing |

### Navigation and story tools

| Feature | Tag | Reason | Evidence | MVP |
|---|---|---|---|---|
| Scene navigator: number, heading, page, length in eighths; click to jump | v1 | FD Go has it; Navigator's core columns | [Navigator FD13][navs13] | partial (no length) |
| Drag scenes to reorder, undoable | v1 | FD's drag cannot be undone | [FD13 guide][fd13] §1.19 | missing |
| Scene metadata: title, summary, colour | v1 (model + preserve) | Summaries in 4 files; colour unused but cheap | [corpus][corpus] | missing (summaries leak into Action) |
| ScriptNotes on a paragraph or range: title, body, colour, type, author; never printed | v1 | 3/14 projects; stored three different ways across versions | [ScriptNotes FD13][sn13]; [corpus][corpus] | partial (Fountain notes only) |
| Bookmarks | v1 | Cheap, documented core | [FD13 guide][fd13] §1.10.6 | missing |
| Character table with dialogue and scene counts | v1 | Cheap; feeds reports and rename | [Navigator Characters FD13][navc13] | missing |
| Beat Board, Outline Editor, Index Cards, Scene View, Structure Lines | later, v1 (preserve) | Loved, but Beat Board items in the corpus are seed content only (4/14 projects) | [Beat Board FD13][bb13]; [corpus][corpus] | missing |
| Character traits, inclusivity charts, arc beats | later, v1 (preserve) | Arc beats carry text in 11 files | [corpus][corpus] | missing (arc-beat paragraphs leak) |
| Split view | later | One panel plus a navigator is enough | [FD13 guide][fd13] §1.8 | missing |

### Production

| Feature | Tag | Reason | Evidence | MVP |
|---|---|---|---|---|
| Scene numbering: number, renumber, keep existing, A-suffix inserts, prefix scheme, left and right toggles, edit one number | v1 | 12/14 projects numbered; 6/14 use letters; schemes "1A" (70 files) and "A2" (4) | [FD13 guide][fd13] §1.13; [corpus][corpus] | partial (number, keep, clear) |
| Omit and unomit a scene, keeping its number and hidden content | v1 | 4/14 projects; FD's encoding is now observed | [FD13 guide][fd13] §1.13.3; [corpus][corpus] | missing |
| Revision sets: name, mark, text style, page colour; active set; revision mode; mark and clear revised | v1 | 9/14 projects; 15,415 revised runs | [Revisions FD12][rev12]; [corpus][corpus] | missing |
| Revision output: margin marks, revised-page header label, page colours, revised text style | v1 | How revisions are issued | [Revisions FD12][rev12]; [Inkwell][inkwell] | missing |
| Deleted-text revision markers | v1 | 21,002 markers in 36 files; 8/14 projects | [corpus][corpus] | missing |
| Display modes: active set and collated | v1; other modes later | "Active" in 100 files, "All" in 3 | [FD13 guide][fd13] §1.13.4; [corpus][corpus] | missing |
| Locked pages: preserve the table and render locked page numbers | v1 | Populated in 5 files, 3/14 projects | [corpus][corpus] | missing |
| Lock pages and A-page numbering (A, AA, merged "2-3"); lock and unlock A-pages | v1 (last production milestone) | Table stakes for production; no A-pages anywhere in the corpus | [Locked Pages FD12][lock12] | missing |
| Locking repair tools (adjust, edit page number, relock from caret) | later | Rarely needed | [Locked Pages FD12][lock12] | missing |
| Dual dialogue: parse, paginate, export; toggle command; inline edit | v1 | 14/14 projects, 115 blocks | [Dual Dialogue FD12][dual12]; [corpus][corpus] | partial (no toggle or edit UI) |
| Alternate dialogue | v1 (preserve), later UI | The collection is empty in all 74 files that have it | [Alt Dialogue FD12][alt12]; [corpus][corpus] | missing |
| Breakdown tagging | later, v1 (preserve) | 0 tags in the 16 files with tag data; partly Method Scheduling's job | [Tagging FD13][tag13]; [corpus][corpus] | missing |
| Revised-pages-only and page/scene-range PDF; clean "white" copy | v1 | How revisions are distributed | [Filmustage][filmustage]; [Print FD12][tp12] | missing |
| Character sides, odd/even, back-to-front | later | Useful, not blocking | [Character Sides][sides] | missing |
| Text-to-speech cast and voices | skip, v1 (preserve) | OS voices; data sits in every file | [FDX notes][fdxnotes] Q1 | missing |
| Real-time collaboration, Cloud | skip | Needs a hosted relay service | [Collaboration FD12][collab12] | — |

### Files, output and interchange

| Feature | Tag | Reason | Evidence | MVP |
|---|---|---|---|---|
| Fountain read and write | v1 | Native format; FD offers no Fountain export | [Fountain](https://fountain.io/syntax/) | done |
| FDX read, Versions 1–6, FD, Fade In and converter dialects | v1 | Real files: V1 ×30, V2 ×5, V3 ×58, V4 ×11, V5 ×7 | [corpus][corpus]; [S32] | partial (parses, with bugs) |
| FDX lossless round-trip: unknown sections and attributes kept as opaque XML | v1 | Users expect Beat Board, cast and settings back | [FDX notes][fdxnotes] Q1, Q6 | missing |
| FDX write with full settings (element specs, layout, header, mores, scene-number options, SmartType, revisions) | v1 | Without them FD applies its own defaults | [FDX notes][fdxnotes] Q2 | partial (body and title page only) |
| .fdxt template read and write | v1 | Same code path as FDX | [FD13 guide][fd13] §1.5 | missing |
| PDF export: embedded font, selectable text, bookmarks, title page toggle | v1 | Universal | [FD13 guide][fd13] §1.5 | partial (no title-page toggle) |
| Plain-text export | v1 | Cheap | [FD13 guide][fd13] §1.5 | partial (CLI dump only) |
| Structured export for Method Scheduling (scenes, INT/EXT, set, DAY/NIGHT, eighths, cast) | v1 | Suite synergy; plain FDX already serves Movie Magic and StudioBinder | [Storiara](https://storiara.com/compare/studiobinder-vs-movie-magic) | missing |
| PDF import | later | FD needed until version 12; a separate effort exists | [FD12 notes][n12] | missing |
| TXT, RTF, Word and Celtx import with a reformat palette | later | The user holds 32 Celtx files; FD routes others through PDF | [import KB](https://kb.finaldraft.com/hc/en-us/articles/15575252515988-What-file-formats-can-I-import-into-Final-Draft); [corpus][corpus] | missing |
| RTF, HTML, CSV and tab-delimited dialogue exports; combine files | later | Low demand | [FD13 guide][fd13] §1.5 | missing |
| .fdr, .fcf, .sex, Avid text | skip | Legacy binary, undocumented or vendor-specific | [FD13 guide][fd13] §1.5 | — |
| Print through the OS | later | PDF covers it | [spec][spec] §9 | missing |

### Reports, tools and the app itself

| Feature | Tag | Reason | Evidence | MVP |
|---|---|---|---|---|
| Scene report (number, intro, location, time, page, eighths, characters) | v1 | High value, cheap; feeds scheduling | [Basic Reports FD12][rpt12] | missing |
| Location, character and cast reports, with CSV | v1 | Expected of a professional writing app | [Basic Reports FD12][rpt12] | missing |
| Statistics, script, note and tag reports | later | Less used | [Basic Reports FD12][rpt12] | missing |
| Format assistant, script compare, reformat palette | later | Useful after imports and for revisions | [Reformat FD12][ref12] | missing |
| Writing stats, sprint timer, typewriter, focus mode | later | FD13 headline items, not essential | [Writing Stats FD13][ws13] | missing |
| Names database, Grammarly, partner links, AI writing | skip | Licensed data or vendor ties | [FD13 guide][fd13] §1.12 | — |
| Atomic save; refuse oversized or hostile files without crashing | v1 | Answers the top complaint | [spec][spec] §2 | done |
| Autosave plus timestamped backup history with a cap | v1 | FD keeps up to 1,000 backups by default | [Auto Save FD12][as12] | partial (60 s recovery copy) |
| Commands with ids, CLI, control channel | v1 | Differentiator for agents and automation | [spec][spec] §2 | done |
| Identical output on macOS, Windows and Linux | v1 | Answers FD's platform gaps | — | done (CI on three OSes) |
| Templates: user-saved, plus own originals (US feature, one-hour TV, multi-cam) | v1 | How users carry formats; never copy FD's | [FD13 guide][fd13] §1.5.2 | missing |
| Preferences: writer name and colour, auto-capitalise, appearance | v1 | Used by revisions and notes | [FD13 guide][fd13] §1.4 | partial |
| Unicode, bidi-safe storage; RTL layout | v1 storage, later layout | RTL is a known unmet need | — | partial |
| Keyboard-complete UI and screen-reader basics | v1 baseline | No screenwriting app we looked at documents it | — | partial (AccessKit on) |
| Stage play, graphic novel, vertical drama templates; iPad app | later | Niche or separate platform | [FD13.3 notes][n133] | — |

Counting the tables, 85 features carry a v1 tag, including the "preserve" rows. Of those, 13 are done, 24 partial and 48 missing. The missing set clusters in three places: FDX fidelity, production, and the editing tools (find, rename, spell check, clipboard).

## The spec needs eleven corrections, and the importer has six bugs

Most corrections come from settings data that Final Draft writes into its own files, so they carry more weight than blog approximations. The corpus measures space before in points: 24 points before headings and shots is 2 lines, and 12 before action and cues is 1. It measures indents in inches from the left edge of the paper ([corpus][corpus]). These values are not on the MVP's 0.1-inch character grid (7.12", 7.38", −0.10"). So the layout module must move to real units, points or hundredths of an inch, before it can honour imported element settings ([layout.rs][layout]).

| # | Spec today | Correct to | Evidence |
|---|---|---|---|
| 1 | 55 lines per page | **54** on Normal (9" body at 6 lines per inch); presets later | [lines-per-page KB][kblines] |
| 2 | 1 blank line before scene headings and shots | **2** before both; store space before in points | [two-lines KB][kb2lines]; 105/109 files ([corpus][corpus]) |
| 3 | Character at 3.7" | **3.50" to 7.25"** | 105/109 files ([corpus][corpus]) |
| 4 | Parenthetical at 3.1", continuation +1 char | **3.00" to 5.50"** with first line at **−0.10"** (−0.12" in 11 files) | [corpus][corpus] |
| 5 | Transition right-aligned to 7.5" | Right-aligned in **5.50" to 7.12"** (7.10" in older files) | [corpus][corpus] |
| 6 | Right scene number at 7.75" | **7.38"** (left 0.75" confirmed) | 108/109 files ([corpus][corpus]) |
| 7 | Enter after Dialogue gives Character | **Action** | 109/109 files ([corpus][corpus]); [FD13 guide][fd13] §1.26.1 |
| 8 | Enter on an empty element converts it to a fixed type | **Opens the element picker** | [Quick Start FD13][qs13] |
| 9 | Tab in Character gives Action | **Transition, and only when the cue is empty**; an option sends Tab to the extension list | [FD13 guide][fd13] §1.26.2 |
| 10 | Automatic (CONT'D) deferred | **v1**, on by default | 105/109 files ([corpus][corpus]); [Mores KB][kbmores] |
| 11 | FDX: "unknown parts ignored" | **Preserve all unknown parts; write Outline/Summary, not Section/Synopsis** | [FDX notes][fdxnotes] Q6 |

Two spec break rules are unverified rather than wrong. The rule that a parenthetical is never the last line before (MORE) conflicts with one practitioner's account, which says Final Draft allows it ([Talentville][talentville]). Final Draft publishes no "at least 2 lines on each side" minimum. Both should be settled from the ground-truth PDFs, as described below, not from Final Draft 11. The spec also needs two additions: scene CONTINUEDs, which cost a line at the top and bottom of every continued page, and a revision-mark column, which the official video shows defaulting to 7.75" ([Revisions FD12][rev12]). The page-number position (7.5" in the spec) is consistent with the FD blog. FD's header data, though, uses a right tab at 7.25" with a negative right indent ([corpus][corpus]). Measuring the 16 PDFs will settle it.

The importer bugs come from running the current converter on the 32 public samples ([FDX notes][fdxnotes] Q6) and from reading [fdx.rs][fdxrs]. The most serious one is structural. The `region()` check accepts a `Paragraph` at any depth under `Content`. So paragraphs nested inside scene summaries, character arc beats, script notes and omitted-scene wrappers become printed Action, placed before their parent. In one public sample this added 76 blank action lines ([S11]); in another, a script note printed as action above the line it annotated ([S16]). The real corpus holds 2,846 such nested paragraphs in 22 files ([corpus][corpus]). The second bug is that `kind_from_fdx` turns every unknown type into Action. Outline and Summary paragraphs, which Final Draft hides, then print, and so do New Act, End of Act, General and Cast List ([S17]). Third, Final Draft 13's empty, self-closing duplicate paragraphs become blank forced-action lines (18 in [S17]). Fourth, run styles other than bold, italic and underline are dropped, so 4,830 AllCaps runs import in lowercase. Fifth, per-paragraph overrides, element settings and page layout are dropped, so any file with non-default stationery paginates on the wrong geometry. Sixth, revision IDs, deleted-text markers and omitted-scene wrappers vanish silently, which is data loss in 9 of 14 projects. The exporter has matching gaps. It writes Version 5 with only the body and title page. It uses the types `Section` (with a `Level` attribute) and `Synopsis`, which Final Draft does not define. It writes Fountain notes as literal bracketed text, and it adds a private attribute to title-page paragraphs ([FDX notes][fdxnotes] Q6).

## Most of the parity proof needs no copy of Final Draft

The decisive insight is that **every FDX saved by Final Draft records its own pagination**. Each scene heading carries `SceneProperties@Page` and a length in eighths, written by Final Draft at save time. In the 16 exact pairs, those cached pages matched Final Draft's PDF on every scene, and 14 of the 16 pairs were saved within about a day of each other ([corpus][corpus]). So the corpus offers two oracles. The first is 16 FDX+PDF pairs: 973 aligned scenes over page counts of 10 to 109, in FDX Versions 1, 2, 4 and 5. The second is the scene-page caches in all 109 complete saves, covering 9,151 headings and 79 feature-length files. The caches are weaker evidence, because the file may have been edited after Final Draft last paginated it. Public FD-authored samples carry the same caches and can be used openly ([FDX notes][fdxnotes] Q5).

The test plan has five layers. **Layer 1 is committed golden tests** for each break rule, updated to 54 lines and 2-line headings. They should be joined by synthetic stress scripts the project writes itself: speeches that end exactly at line 52, 53 and 54, a parenthetical at the bottom of a page, a heading followed by one line, dual dialogue across a break, and long unbroken words. These are clean-room by construction. They also become the input batch for Final Draft 11 later. **Layer 2 is public-sample conformance.** The 32 permissively licensed FDX files (16 written by Final Draft) are fetched by a `cargo xtask fetch-samples` step into `target/`, pinned by URL and SHA-256 and skipped when offline. They are not committed, because several MIT repositories contain copyrighted scripts ([FDX notes][fdxnotes] Q5). The tests check that every file parses, that the number of printed elements equals the number of visible top-level paragraphs, that unknown sections survive a round-trip unchanged after XML canonicalisation, and that our scene pages match the cached pages in the FD-authored files.

**Layer 3 is the private corpus harness**, which is the heart of the plan. It must keep the maintainer's scripts, and possibly other people's, entirely local. The harness reads a manifest from a path set by an environment variable (for example `STORYTELLER_CORPUS`), stored outside the repository. It also refuses to run if that path is inside the git work tree. A `cargo xtask corpus` command, deliberately not part of `cargo xtask ci`, imports each FDX, paginates it, and compares three things. It compares each scene's start page with the PDF's page and with Final Draft's cached page. It compares the total page count. And it compares where each page's first and last lines fall, using hashes of the `pdftotext -layout` lines, never the text itself. Output is aggregate only: anonymous pair numbers, the percentage of scenes exact, the maximum drift, and the first divergent page as a number. Nothing that could leak a title, name or line goes into logs, issues or commits. A CI guard should fail if any `.fdx` or `.pdf` outside an allow-listed fixtures folder is ever tracked. The same harness can settle the open layout questions empirically, because measuring public output is clean-room. It can measure the longest action and dialogue lines across all PDFs, which gives characters per line and so the Courier Final Draft width question. It can collect every dialogue split, recording lines before (MORE) and after (CONT'D) and whether a parenthetical ever ends a page. It can find the shortest content after a heading at a page bottom. And `pdftotext -bbox` gives the positions of page numbers, scene numbers, (MORE) and revision marks. It must never extract the font embedded in Final Draft's PDFs. The exit target for pagination is **100% scene-exact on the 16 pairs**. A good second target is 95% or more against the caches of the other feature-length files. Fade In's own FAQ calls exact parity hard, because Final Draft's layout has content-dependent quirks that vary by version ([Fade In KB][fadeinkb]). The remaining misses should be catalogued, not forced.

**Layer 4 is hostile-input fuzzing**, which already exists, extended to the new FDX sections. **Layer 5 is behaviour tests through the control channel**: keystroke scripts that drive Tab, Enter, the picker and SmartType, and check the resulting element sequence and the status-bar hint. Screenshots via `ui.screenshot` cover visual checks ([control protocol](../control-protocol.md)).

Only a short list of questions genuinely needs a licensed Final Draft 11, and each one can be prepared as a file or a click script in advance.

| # | Question for Final Draft 11 | How to run it |
|---|---|---|
| 1 | Does it open our exported FDX (Version 4, 5 and minimal), which defaults does it apply, and what does it drop? This fixes the export `Version` and confirms the inferred mapping of format Version 4 to FD11 | Open 6 prepared exports; save each back as FDX and PDF; diff |
| 2 | Encodings never seen populated: alternates, tags placed in text, A-page entries in locked pages, an omitted scene's stored content, an inline image | In one throwaway script, add one of each and save |
| 3 | A-page and A-scene lettering beyond the narrated examples: letters past Z, whether I and O are skipped, prefix schemes, merged "2-3" pages, relock and unlock | Lock a prepared script; apply a scripted list of edits; save PDF after each |
| 4 | Pagination cases the corpus lacks: dual dialogue at a break; scene CONTINUED combined with (MORE); whether automatic and page-break CONT'D stack; the Tight, Loose and Very Tight line heights | Render the synthetic stress batch to PDF |
| 5 | Any break-rule threshold the corpus PDFs leave ambiguous | Same batch, targeted cases |
| 6 | Undocumented editor keys: Tab from New Act, End of Act, Cast List or a non-empty cue; Shift-Tab | Short keystroke checklist |
| 7 | Revision output details: header label placement, mark column, deleted-text mark position, page colour in PDF | Revise the prepared script in two sets; save PDF |
| 8 | FD11-only constructs (Story Map, Structure Points, General Notes) as written in a Version 4 file | Create one of each; save |

Everything in this list is observation of public output from files we create ourselves, which `docs/spec.md` already allows ([spec][spec] §2). Nothing requires opening Final Draft's bundled templates or binaries.

## Seven milestones, ordered by what breaks trust first

The order follows one principle: never lose a user's data, then never move their page breaks, then make writing feel familiar, then add production authoring. Fidelity comes first because a production user opening a revised shooting script today silently loses its revisions and gets hidden outline text printed. That is worse than any missing feature.

**M1, model and importer repair.** Fix the six importer bugs. Move layout to real units. Add an element-type registry (core types, custom types with a "paginate as" rule, hidden types), run styles (AllCaps, Strikeout, revision ID), per-paragraph overrides, scene properties, and an opaque store for every unknown FDX section and attribute. Apply spec corrections 1–7 and 10. Build the private corpus harness now, so every later change is measured. Exit: no corpus file prints hidden text; all 32 public samples round-trip their unknown sections unchanged.

**M2, pagination parity.** Read element settings, page layout, header and footer, mores and continueds, and scene-number options from FDX. Calibrate characters per line and the break thresholds from the PDFs. Add scene CONTINUEDs and the free-layout title page. Write full settings on export. Exit: 16/16 pairs scene-exact; misses against the cache target catalogued.

**M3, the Final Draft writing feel.** Add the element picker, the 0–9 shortcuts, the status-bar hint, SmartType lists stored in the document with learning and next-speaker guessing, smart quotes, auto-capitalise and int./ext. conversion. Add element-preserving clipboard, find and replace, go to, rename character, spell check, backup history, dark mode and the original templates. Exit: control-channel keystroke suites pass for every documented Tab and Enter transition.

**M4, notes and navigation.** Add ScriptNotes in all three FDX generations, scene titles, summaries and colours, undoable drag-reorder in the navigator, scene length in eighths, bookmarks, the character table, and show and hide for outline paragraphs.

**M5, production I.** Add full scene numbering (A-suffix inserts, prefix scheme, per-side display), omit and unomit, revision sets, revision mode, mark and clear, deleted-text markers, and revision rendering in the editor and PDF. Add header revision labels, revised-pages-only and range PDFs, and clean copies.

**M6, production II and reports.** Render imported locked pages, then add lock pages, A-page generation, lock and unlock A-pages. Add scene, location, character and cast reports with CSV, the Method Scheduling export and plain-text export.

**M7, the Final Draft 11 session and release.** Run the eight prepared checks in one sitting. Turn every answer into a committed synthetic golden test. Fix the differences and ship V1.0. Everything tagged later then forms the V1.x and V2 backlog, led by PDF import and the Beat Board and Outline Editor, whose data V1 already preserves.

## Conclusion

The research shifts the main risk of V1.0 away from feature count and onto fidelity. The editor features users notice are mostly cheap tables and dialogs. What decides adoption is quieter: the file comes back intact, and page 47 is still page 47. The MVP was built the right way for that, pure and deterministic, with golden tests. But it was calibrated against third-party guides, not against Final Draft's own data, so its two largest layout constants are wrong. Final Draft has been writing its own answers into every FDX file all along, in its element settings and its per-scene page caches. That turns the maintainer's private archive into a measuring instrument rather than just test input, provided the harness keeps it local and reports only numbers.

The second takeaway is about scope. Alternates, breakdown tags and A-pages are heavily documented but unused in this corpus. That argues for preserving their data now and building their UI later. It also argues for spending the scarce Final Draft 11 time only on behaviours no file or PDF can reveal. Treated that way, the licensed copy becomes a final audit of perhaps a day, not a development dependency.

[fd13]: https://www.finaldraft.com/downloads/manuals/fd13-user-guide-mac-exported.pdf
[n12]: https://update.finaldraft.com/12/0/release_notes.html
[n13]: https://update.finaldraft.com/13/0/release_notes.html
[n132]: https://update.finaldraft.com/13/2/release_notes.html
[n133]: https://update.finaldraft.com/13/3/release_notes.html
[kblines]: https://kb.finaldraft.com/hc/en-us/articles/15575339757716-How-many-lines-per-page-does-Final-Draft-write-and-what-are-my-line-spacing-options
[kb2lines]: https://kb.finaldraft.com/hc/en-us/articles/15575271650452-Why-are-there-two-blank-lines-above-a-Scene-Heading-and-a-Shot
[kbsentence]: https://kb.finaldraft.com/hc/en-us/articles/15575343691284-I-don-t-want-dialogue-and-action-to-break-at-the-end-of-a-sentence-at-the-bottom-of-the-page
[kbmores]: https://kb.finaldraft.com/hc/en-us/articles/44647566253588-Mores-and-Continueds-settings-in-Final-Draft-Writer
[fadeinkb]: https://www.fadeinpro.com/kb/content/3/29/en/why-is-my-script-a-different-length-than-in-final-draft.html
[talentville]: https://www.talentville.com/snippet/screenplay-page-break-rules
[inkwell]: https://www.inkwell.software/blog/screenplay-revisions-locked-pages-and-a-pages
[filmustage]: https://filmustage.com/blog/script-revision-colors-markup-history/
[qs13]: https://www.youtube.com/watch?v=gd0oc1RQsGw
[qs12]: https://www.youtube.com/watch?v=F4Uj_L6K56c
[s123]: https://www.youtube.com/watch?v=Zhxw-pDGUbk
[oel13]: https://www.youtube.com/watch?v=nk73fnWxx_s
[navs13]: https://www.youtube.com/watch?v=-fcvGetFg4g
[navc13]: https://www.youtube.com/watch?v=TUIs7PsAudU
[sn13]: https://www.youtube.com/watch?v=IUxYt0UrnI8
[bb13]: https://www.youtube.com/watch?v=3eUwfuu2IcI
[tag13]: https://www.youtube.com/watch?v=L_eSP9vCsf8
[ws13]: https://www.youtube.com/watch?v=tis7MnjQc7o
[tw13]: https://www.youtube.com/watch?v=Xcb8j5rKfto
[mm13]: https://www.youtube.com/watch?v=STrwiTAtJxQ
[sides]: https://www.youtube.com/watch?v=L795WRxtbXU
[rev12]: https://www.youtube.com/watch?v=mwlxQVmi2tU
[lock12]: https://www.youtube.com/watch?v=J3fU9beJxHo
[tc12]: https://www.youtube.com/watch?v=2owyLjFylqM
[collab12]: https://www.youtube.com/watch?v=KaNDvz-ENA4
[alt12]: https://www.youtube.com/watch?v=W5uKeZ9BZDQ
[dual12]: https://www.youtube.com/watch?v=nI36Ct9ycQQ
[tp12]: https://www.youtube.com/watch?v=vIVBLrZPLms
[ce12]: https://www.youtube.com/watch?v=aTGzdXgp_VQ
[ref12]: https://www.youtube.com/watch?v=oMLL6bD-o5M
[rpt12]: https://www.youtube.com/watch?v=Nd5lsnfZwps
[as12]: https://www.youtube.com/watch?v=JPCrWx8gOfs
[S11]: https://github.com/extratone/corn/blob/main/docs/Children%20of%20The%20Corn%2030.fdx
[S16]: https://github.com/localgpt-app/localgpt/blob/main/crates/previs/samples/cold-eggs.fdx
[S17]: https://github.com/regosen/Phil-Ochs-screenplay/blob/main/The%20War%20Is%20Over.fdx
[S32]: https://github.com/wildwinter/screenplay-tools/blob/main/tests/TestFDX-FI.fdx
[corpus]: #about-the-private-corpus
[fdxnotes]: v1-notes/fdx-format.md
[spec]: ../spec.md
[model]: ../../crates/core/src/model.rs
[flow]: ../../crates/engine/src/flow.rs
[layout]: ../../crates/core/src/layout.rs
[fdxrs]: ../../crates/core/src/fdx.rs
[paginate]: ../../crates/core/src/paginate.rs
[commands]: ../../crates/engine/src/commands.rs
