# Final Draft XML (.fdx) format as seen in the wild

Method: 32 public FDX files from permissively licensed GitHub repos were downloaded to
`_research-media/storyteller/fdx-online/` (the catalogue with URL and licence for each is in `sources.md`
there). They were inventoried with a Python `xml.etree` script (`_research-media/storyteller/fdx-online-inventory/inv.py`,
`vals.py`; full output in `inventory.txt`, `enums.txt`). Of the 32, 16 were written by Final Draft desktop,
1 by Fade In, and the rest by converters, generators or synthetic fixtures (see Q5). Counts like "14/32" below
mean "found in 14 of the 32 files". Where a finding cites a sample file, that file is the evidence. All
structure descriptions are in my own words; no Final Draft binaries or templates were touched, and no GPL/AGPL
code was read.

Shorthand for sample citations (full URLs in Q5 and sources.md):
[S03](https://github.com/45inertia/Screenwriting-Project-Dissertation/blob/main/Documentation/market_research/final_draft/star_wars_test_screenplay.fdx) FD13 ·
[S10](https://github.com/dmongrel/fdx-mcp-server/blob/master/src/fdx/resources/NewFile.fdx) FD13 blank ·
[S11](https://github.com/extratone/corn/blob/main/docs/Children%20of%20The%20Corn%2030.fdx) Version 4 ·
[S17](https://github.com/regosen/Phil-Ochs-screenplay/blob/main/The%20War%20Is%20Over.fdx) FD13 outline-heavy ·
[S01](https://github.com/4444J99/narratological-algorithmic-lenses/blob/main/specs/11-el-series/drafts/2018-03-12-tale-of-el.fdx) Version 3 ·
[S28](https://github.com/SimonGZ/jumpcut/blob/main/tests/fixtures/fdx-import/title-page-cast-page.fdx) Version 4 with revisions ·
[S31](https://github.com/wildwinter/screenplay-tools/blob/main/tests/TestFDX-FD.fdx) Version 5 from FD ·
[S32](https://github.com/wildwinter/screenplay-tools/blob/main/tests/TestFDX-FI.fdx) Fade In ·
[S24](https://github.com/vilcans/screenplain/blob/master/tests/files/dual-dialogue.fdx) screenplain dual dialogue ·
[S26](https://github.com/shivanshv45/Bluepages/blob/main/tests/fixtures/feature-draft-1.fdx) synthetic revisions/omitted ·
[S16](https://github.com/localgpt-app/localgpt/blob/main/crates/previs/samples/cold-eggs.fdx) generated, inline note

---

## Q1. What is the complete structure of FDX: top-level sections, elements, attributes and enumerated values?

### Takeaway
FDX is a flat XML document: a `<FinalDraft>` root, one `<Content>` of `<Paragraph>`s (the script), then
about 30–45 sibling "preamble" sections holding settings, title page, production data, UI state and Beat
Board data. Final Draft writes every attribute every time (all 7 run attributes, all 8 paragraph layout
attributes), with colours as 48-bit `#RRRRGGGGBBBB` hex, booleans as `Yes`/`No`, lengths in inches (indents),
points (spacing/margins) or eighths of a page (scene length). There is no public DTD or XSD.

### Cited Findings

**Root and ordering**
- Root is `<FinalDraft DocumentType="Script" Template="No" Version="N">`. In all 32 samples `DocumentType` is `Script` and `Template` is `No`; `Version` values seen: 1 (9 files, all from converters), 2 (Fade In), 3, 4, 5, 6. The FD declaration is `<?xml version="1.0" encoding="UTF-8" standalone="no" ?>` — [S10](https://github.com/dmongrel/fdx-mcp-server/blob/master/src/fdx/resources/NewFile.fdx), [S32](https://github.com/wildwinter/screenplay-tools/blob/main/tests/TestFDX-FI.fdx) (Fade In leaves out `standalone`).
- Final Draft templates use their own extension, `.fdxt` (FD12+), and are told apart from documents by that extension — [Final Draft KB via search snippet](https://kb.finaldraft.com/hc/en-us/articles/27525594609684-How-do-I-export-a-Final-Draft-file-to-a-different-format-like-RTF-or-TXT). `Template="Yes"` is listed as a possible value by [open-fdx-toolkit FDX_SPEC.md](https://github.com/sfingali/open-fdx-toolkit/blob/main/FDX_SPEC.md) but no sample shows it.
- Order of top-level children in an FD13 (Version 6) file: `DocumentRef, Content, Watermarking, HeaderAndFooter, SpellCheckIgnoreLists, PageLayout, WindowState, TextState, ElementSettings×17, TitlePage, SmartType, MoresAndContinueds, LockedPages, Revisions, SplitState, Macros, Actors, Cast, SceneNumberOptions, CastList, CharacterHighlighting, ScriptNavigatorPreferences, AltCollection, TargetScriptLength, ListItems, DisplayBoards, Writers, WriterMarkup, TagData, Characters, ScriptNotes, Navigator, Images, Outlines` — [S03](https://github.com/45inertia/Screenwriting-Project-Dissertation/blob/main/Documentation/market_research/final_draft/star_wars_test_screenplay.fdx), and the same in all 10 Version-6 samples. Note `Content` comes **first** (after `DocumentRef`); open-fdx-toolkit's spec shows preamble before Content, which none of the FD samples do — [FDX_SPEC.md](https://github.com/sfingali/open-fdx-toolkit/blob/main/FDX_SPEC.md).
- Optional extras seen in some files: `UnanchoredScriptNotes` (Version 3/4), `CharacterNavigatorPreferences` (V3/4), `SceneNavigatorPreferences` and `TagsNavigatorPreferences` (V4/5), `DeletedText` (V2/V4) — [S01](https://github.com/4444J99/narratological-algorithmic-lenses/blob/main/specs/11-el-series/drafts/2018-03-12-tale-of-el.fdx), [S11](https://github.com/extratone/corn/blob/main/docs/Children%20of%20The%20Corn%2030.fdx), [S28](https://github.com/SimonGZ/jumpcut/blob/main/tests/fixtures/fdx-import/title-page-cast-page.fdx).

**`Content/Paragraph`** (the script body)
- Attributes FD writes: `Type`, `Alignment` (`Left|Center|Right|Full`), `FirstIndent`, `LeftIndent`, `RightIndent` (inches, two decimals, e.g. `1.50`, `7.50`), `Leading` (`Regular`), `SpaceBefore` (points: `0|12|15|24`), `Spacing` (`1|2`, line spacing multiple), `StartsNewPage` (`Yes|No`), `Number` (scene number, scene headings only), `id` (UUID, Version 6), `Label` (e.g. `(Ch. #1)`), and on nested paragraphs `OutlineLevel` — inventory across samples, e.g. [S17](https://github.com/regosen/Phil-Ochs-screenplay/blob/main/The%20War%20Is%20Over.fdx), [S11](https://github.com/extratone/corn/blob/main/docs/Children%20of%20The%20Corn%2030.fdx). In body paragraphs FD13 usually writes only `Type` (+`id`, `Number`) and leaves layout to `ElementSettings`; layout attributes appear on a paragraph only when the user overrode them (76 of ~10 000 paragraphs).
- `Type` values seen in bodies: `Action, Dialogue, Character, Scene Heading, Parenthetical, Transition, General, Summary, Outline 1, Outline 2, Shot, New Act, End of Act, Act&Scene Break, Section Heading, Chapter, End of Chapter` — inventory (`enums.txt`). `ElementSettings@Type` adds `Cast List, Sequence, Note, Outline 3, Outline 4, Outline Body, Title`. Other names reported by open-fdx-toolkit (from rsdoiel's files): `Beat, Center, Right, Last Revised, Page #, Script, StoryMap, Normal Text`, plus `Lyrics` — [FDX_SPEC.md](https://github.com/sfingali/open-fdx-toolkit/blob/main/FDX_SPEC.md); a `Lyrics` ElementSettings entry appears in [EricQXu/Overlay_AR](https://github.com/EricQXu/Overlay_AR/blob/HEAD/Overlay_AR.fdx) (no licence, inspected only). **Element types are user-definable**: the type list in a file is whatever its ElementSettings define (TV/stage templates add `New Act`, `End of Act`, `Chapter`, …), so an importer must accept unknown `Type` strings and map them through `ElementSettings/Behavior@PaginateAs`.
- Paragraph children: `Text` runs; `SceneProperties` (scene headings); `DualDialogue`; `ScriptNote` (inline note, seen only in synthetic/generated files [S16](https://github.com/localgpt-app/localgpt/blob/main/crates/previs/samples/cold-eggs.fdx), [S26](https://github.com/shivanshv45/Bluepages/blob/main/tests/fixtures/feature-draft-1.fdx), and in the open-fdx-toolkit spec's FD11/12 example); `Picture Id= Width= Height=` (image placed in the script, refers to `Images/Image@Id`) — [S03](https://github.com/45inertia/Screenwriting-Project-Dissertation/blob/main/Documentation/market_research/final_draft/star_wars_test_screenplay.fdx).
- FD13 writes empty self-closing paragraphs such as `<Paragraph Type="Outline 1" id="X"/>` immediately followed by a non-empty paragraph with the same `id` — [S17](https://github.com/regosen/Phil-Ochs-screenplay/blob/main/The%20War%20Is%20Over.fdx) lines 9–10. Purpose unknown (see Gaps).

**`Text` runs**
- Attributes: `AdornmentStyle` (`0` or `-1`), `Background`, `Color` (48-bit hex `#RRRRGGGGBBBB`; FD also writes the literal placeholder `#RRRRGGGGBBBB` in some places), `Font` (`Courier Final Draft`, `Courier Prime`, `Courier`, `Arial`, `Helvetica Neue`), `RevisionID` (`0` = unrevised; `1..n` = revision set ID), `Size` (points), `Style` — inventory. In FD13 bodies a run usually has no attributes at all, or only `Style`/`AdornmentStyle`; the full set appears in title pages, headers, notes and Beat Board text — [S17](https://github.com/regosen/Phil-Ochs-screenplay/blob/main/The%20War%20Is%20Over.fdx).
- `Style` is a `+`-joined set. Tokens seen: `Bold, Italic, Underline, AllCaps, HiddenText` (e.g. `Bold+Underline+AllCaps`, `Bold+HiddenText`, `Italic+HiddenText`) — inventory. `Strikeout` is listed as unconfirmed by [FDX_SPEC.md](https://github.com/sfingali/open-fdx-toolkit/blob/main/FDX_SPEC.md). Empty string means plain.
- A paragraph's text is the concatenation of its `Text` runs in order; runs split wherever style, font or revision changes — [FDX_SPEC.md](https://github.com/sfingali/open-fdx-toolkit/blob/main/FDX_SPEC.md), [S28](https://github.com/SimonGZ/jumpcut/blob/main/tests/fixtures/fdx-import/title-page-cast-page.fdx).

**`SceneProperties`** (first child of a `Scene Heading` paragraph)
- Attributes: `Length` (eighths of a page as a mixed fraction: `1/8`, `7/8`, `1`, `1 2/8`, `0/8` for omitted scenes; scenes longer than a page are **not** capped at `8/8` — values like `1 4/8` occur), `Page` (starting page as a string), `Title` (scene title, usually empty), `Color` (scene colour, e.g. `#FFFF00000000`) — [S11](https://github.com/extratone/corn/blob/main/docs/Children%20of%20The%20Corn%2030.fdx), inventory. This contradicts the open-fdx-toolkit guess that long scenes report `8/8` and its claim of a `Number` attribute on SceneProperties (the scene number is on the Paragraph) — [FDX_SPEC.md](https://github.com/sfingali/open-fdx-toolkit/blob/main/FDX_SPEC.md).
- Children: `Summary` (scene synopsis: holds full `Paragraph/Text`) and `SceneArcBeats` holding `CharacterArcBeat Name="CHARACTER"`, each with a full `Paragraph/Text` for that character's beat in the scene (often empty) — [S11](https://github.com/extratone/corn/blob/main/docs/Children%20of%20The%20Corn%2030.fdx); the `Summary/Paragraph/Text` path is also in the [surrealroad XPath gist](https://gist.github.com/surrealroad/effaa4f84d8ba53cecb6) and [FDX_SPEC.md](https://github.com/sfingali/open-fdx-toolkit/blob/main/FDX_SPEC.md).
- Final Draft stores computed values (Length, Page) in the file; they are caches, recomputed by FD on open (inference from them being present on every heading).

**Title page**
- `TitlePage` holds `Content` of `Paragraph`s (no `Type`, or `Type` absent; attributes `Alignment` `Center|Left|Full|Right`, indents, `SpaceBefore`, `StartsNewPage` for multi-page front matter), a `HeaderAndFooter` of its own, and a `TextState` — [S28](https://github.com/SimonGZ/jumpcut/blob/main/tests/fixtures/fdx-import/title-page-cast-page.fdx). Title-page paragraphs can carry `Tabstops/Tabstop Position= Type=` (`Left|Center|Right`) — [S28](https://github.com/SimonGZ/jumpcut/blob/main/tests/fixtures/fdx-import/title-page-cast-page.fdx). There are **no semantic fields** (title/author/contact): the title page is free layout text; position is the only meaning.

**Settings sections**
- `ElementSettings Type="X"` (one per element type, 9–18 per file) with `FontSpec` (run attributes), `ParagraphSpec` (Alignment, FirstIndent, Leading, LeftIndent, RightIndent, SpaceBefore, Spacing, StartsNewPage), `Behavior` (`PaginateAs`, `ReturnKey` = next element on Enter, `TabKey`, `Shortcut`, `WinShortcut`) and `Outline` (`CanHide`, `Level`) — [S31](https://github.com/wildwinter/screenplay-tools/blob/main/tests/TestFDX-FD.fdx) lines 116–121. `Outline@Level` values 0–7 drive the outline/Beat Board lanes.
- `PageLayout` (`TopMargin`, `BottomMargin`, `HeaderMargin`, `FooterMargin` in points; `BackgroundColor`, `ForegroundColor`, `InvisiblesColor`, `DocumentLeading` `Normal`, `BreakDialogueAndActionAtSentences`, `UsesSmartQuotes`) with `PageSize Width= Height=` (inches, `8.50`×`11.00`) and `AutoCastList` (`AddParentheses`, `AutomaticallyGenerate`, `CastListElement`) — inventory.
- `HeaderAndFooter` (`HeaderVisible`, `FooterVisible`, `HeaderFirstPage`, `FooterFirstPage`, `StartingPage`) containing `Header/Paragraph` and `Footer/Paragraph` with `Text`, `Tabstops`, and `DynamicLabel Type=` (`Page #`, `Collated Revisions`, `Last Revised`, `Date`, `Label`) — inventory.
- `MoresAndContinueds`: `FontSpec`, `DialogueBreaks` (`AutomaticCharacterContinueds`, `BottomOfPage`, `DialogueBottom="(MORE)"`, `DialogueTop="(CONT'D)"`, `TopOfNext`), `SceneBreaks` (`ContinuedNumber`, `SceneBottom="(CONTINUED)"`, `SceneBottomOfPage`, `SceneTop="CONTINUED:"`, `SceneTopOfNext`) — inventory.
- `SceneNumberOptions` (`NumberScheme="1A"`, `ShowNumbersOnLeft`, `ShowNumbersOnRight`, `LeftLocation`, `RightLocation` in inches) + `FontSpec` — inventory.
- `SmartType`: `Characters/Character`, `Extensions/Extension`, `SceneIntros/SceneIntro` (with `Separator=". "`), `Locations/Location`, `TimesOfDay/TimeOfDay` (with `Separator=" - "`), `Transitions/Transition`, each as element text — [S31](https://github.com/wildwinter/screenplay-tools/blob/main/tests/TestFDX-FD.fdx).
- `Macros/Macro` (`Name`, `Text`, `Element`, `Transition`, `Shortcut`) with `Alias` (`Text`, `Confirm`, `MatchCase`, `SmartReplace`, `WordOnly`) and `ActivateIn Element=` — inventory.
- `SpellCheckIgnoreLists`: `IgnoredWords/Word`, `IgnoredRanges/Range Start= End=` (character offsets into the script) — inventory.
- `Watermarking` (`Opacity`, `Position="Diagonal Descending"`) with `Distribution/Name`, `DynamicContent/Paragraph`, `WatermarkImage Height=` — inventory.
- `TargetScriptLength` (empty in samples), `CharacterHighlighting/Character` (`Name`, `Color`, `Visible`) — inventory.

**Production sections**
- `Revisions` (`ActiveSet`, `Location` = mark column in inches, `RevisionMode` `Yes|No`, `RevisionsShown` `Active|All`, `ShowAllMarks`, `ShowAllSets`, `ShowPageColor`) with `Revision` entries (`ID`, `Name`, `Color`, `PageColor`, `Mark="*"`, `Style`, `FullRevision`) — [S28](https://github.com/SimonGZ/jumpcut/blob/main/tests/fixtures/fdx-import/title-page-cast-page.fdx), [S01](https://github.com/4444J99/narratological-algorithmic-lenses/blob/main/specs/11-el-series/drafts/2018-03-12-tale-of-el.fdx).
- `DeletedText/DeletedTextLocation Offset= RevisionID=` — [S28](https://github.com/SimonGZ/jumpcut/blob/main/tests/fixtures/fdx-import/title-page-cast-page.fdx).
- `LockedPages` — always empty in the downloadable samples.
- `TagData` with `TagCategories/TagCategory` (`Id` UUID, `Name`, `Number`, `Color`, `Style="Bold"`), plus `TagDefinitions` and `Tags` (empty in all samples) — [S32](https://github.com/wildwinter/screenplay-tools/blob/main/tests/TestFDX-FI.fdx). Default categories include Cast Members, Background Actors, Stunts, Vehicles, Props, Special Effects, Wardrobe, Makeup/Hair, Animals, Animal Wrangler, Music, Sound … — inventory.
- `CastList` (`SortOption` `Alphabetical | Order of appearance in each scene`) with `CustomOrder`; `Cast/Member` (`Character`, `Actor`) and `Cast/Narrator Actor=` with `Element Type=`; `Actors/Actor` (`Name`, `MacVoice`, `WinVoice`, `Pitch`, `Speed`) — the latter are text-to-speech ("ScriptCloud"/read-aloud) cast assignments — inventory.
- `AltCollection` — always empty in the samples (see Q3).
- `Characters` (FD11+ character profiles): `CharacterTraitData` with `Traits/Trait` (`ID`, `Name`: Age, Disability, Ethnicity, Gender, Occupation, Orientation, Role; `Type="text"`) and `Holders`, plus `ChartOptions Identifier="InclusivityAnalysis"` with `FilterSets/FilterSet/Filter/Match` — inventory.
- `ScriptNotes/ScriptNote` (Version 5/6: `Id`, `Name`, `Color`, `DateTime`, `DateModified`, `Range="start,end"` character offsets, `RefId` UUID, `Type`, `WriterID`, `WriterName`) with `Paragraph/Text` body — [S03](https://github.com/45inertia/Screenwriting-Project-Dissertation/blob/main/Documentation/market_research/final_draft/star_wars_test_screenplay.fdx) line 975. `UnanchoredScriptNotes` is the older (V3/V4) container.
- `Writers`, `WriterMarkup State="Off"` (collaboration / writer-colour markup) — inventory.
- `Images Hidden=` with `Image Id= Path= Length=`; the element text is the image data in a text encoding (64 622 chars for a 51 736-byte JPEG, a ratio that suggests Ascii85) — [S03](https://github.com/45inertia/Screenwriting-Project-Dissertation/blob/main/Documentation/market_research/final_draft/star_wars_test_screenplay.fdx).
- `DocumentRef DateTime= id=` with `XRef DateTime= id=` children (Version 6 only; looks like document identity/lineage) — [S17](https://github.com/regosen/Phil-Ochs-screenplay/blob/main/The%20War%20Is%20Over.fdx).

**Beat Board / Story Map / outline**
- `ListItems/ListItem` (`Id`, `Type` `Beat|PeerLink`, `Title`, `Color`; for `PeerLink` also `StartPoint`, `EndPoint` item ids and `FrontCap`/`EndCap` `Arrow|None`) with a `Content/Paragraph/Text` body, and optional `SubordinateTo Type="PageGoal" Level="Outline 1" Number= Length= Order=` (a beat's place on the Story Map timeline in pages) — [S17](https://github.com/regosen/Phil-Ochs-screenplay/blob/main/The%20War%20Is%20Over.fdx), [S03](https://github.com/45inertia/Screenwriting-Project-Dissertation/blob/main/Documentation/market_research/final_draft/star_wars_test_screenplay.fdx).
- `DisplayBoards/DisplayBoard` (`Id`, `Type` `Beat|StoryMap`, `Width`, `Height`, `ZoomLevel`, `ScrollOrigin`) with `Item` placements (`Id` = ListItem id, `LaneId`, `Left`, `Top`, `Width`, `Height`) and `Lanes/Lane` (`Id`, `Label`, `Type` `Outline 1|Outline 2`, `Level`) — inventory.
- `Outlines` (`Paragraphs="Show"`, `StructureLines`, `Hidden`) — Version 5/6 — inventory.

**Pure UI state (safe to drop or regenerate)**
- `WindowState` (`Left`, `Top`, `Width`, `Height`, `Mode`), `TextState` (`Scaling`, `Selection="a,b"`, `ShowInvisibles`), `SplitState` (`ActivePanel`, `SplitMode`, `SplitterPosition`, `CardsAcross`, `ShowAction`, `ShowSceneTitle`, `ShowSummary`) with `ScriptPanel`/`BeatBoardPanel`/`IndexCardPanel` (`DisplayMode` `Page|Normal|BeatBoard|Summary`, `FontSpec`), `Navigator/Tabs/Tab`, `*NavigatorPreferences` column settings — inventory.

### Inferences
- A round-tripping importer should keep every unknown element/attribute as opaque XML, because many sections (Beat Board, cast, character traits, TTS actors, writers, images) have no screenplay meaning but users expect them back.
- 48-bit colours: `#FFFF00000000` is red; convert to 8-bit by taking the high byte of each 16-bit channel.
- Offsets in `Range`, `IgnoredRanges` and `DeletedTextLocation@Offset` are character positions into the flattened script text, so editing text without rewriting them corrupts anchors. A writer that cannot recompute them should drop those items rather than write stale offsets.
- FD13 bodies with almost no attributes mean layout must come from `ElementSettings` + `PageLayout`, not from paragraphs.

### Gaps
- No public DTD/XSD; Final Draft says it shares specs with partners only ([Cast & Crew interview](https://blog.castandcrew.com/final-draft-interview)). All of the above is observed, not specified.
- Meaning of `AdornmentStyle` (`0` vs `-1`), the empty duplicate-`id` paragraphs in FD13, `Holders`, `Navigator` tabs, and `TargetScriptLength` contents not established.
- `Strikeout`, `Subscript`/`Superscript` style tokens, and `Lyrics` as a body type not seen in any downloaded FD file.

---

## Q2. What changed between FD 8, 9, 10, 11, 12 and 13?

### Takeaway
The `Version` attribute is a file-format revision, not the product number. From dated files and section
appearance, the likely mapping is Version 1–2 ≈ FD8/9 era, 3 ≈ FD10, 4 ≈ FD11, 5 ≈ FD12, 6 ≈ FD13. This is
an inference; no source states it. Each version adds top-level sections; none were removed, so a reader
should be lenient and a writer should pick one target version.

### Cited Findings
- FDX (XML) replaced the binary `.fdr` with Final Draft 8 in 2009 — [Cast & Crew interview with Joe Jarvis](https://blog.castandcrew.com/final-draft-interview); [PRONOM](https://www.nationalarchives.gov.uk/PRONOM/Format/proFormatSearch.aspx?status=detailReport&id=2697) says FD8+ writes an XML text file.
- FD11/12 open FD8–10 files directly; FD13 opens FD5–12; FD10–13 cannot save `.fdr` — [Final Draft KB (search snippet)](https://kb.finaldraft.com/hc/en-us/articles/15574773633428-How-can-I-get-a-script-written-in-Final-Draft-10-9-8-into-Final-Draft-12-11), [KB FD13](https://kb.finaldraft.com/hc/en-us/articles/15575229240340-How-can-I-get-a-script-written-in-an-earlier-version-of-Final-Draft-into-Final-Draft-13). (KB pages returned 403 to direct fetch; only search snippets were read.)
- Final Draft Writer (the newer lightweight app) refuses FDX files that use Track Changes, Revisions, Alt Dialogue, Omitted Scenes, Locked Pages, Tags or Images in the script, and only opens files last saved in FD13 — [Final Draft KB (search snippet)](https://kb.finaldraft.com/hc/en-us/articles/45143952909972-When-I-try-to-open-an-FDX-in-Final-Draft-Writer-I-get-this-error-Unsupported-FDX-This-file-contains-features-that-prevent-it-from-opening-at-this-time), [KB](https://kb.finaldraft.com/hc/en-us/articles/45802394750740-Why-does-Final-Draft-Writer-give-me-an-error-message-when-I-try-to-open-a-file-from-an-older-version-of-Final-Draft). This list is a useful checklist of "production features" stored in FDX.
- Sections by `Version` in the samples (inventory, `vals.py`):
  - Version 2 (Fade In 2.x+ export): Content with `Number` on Paragraph, sparse attributes, `TagData` with empty `TagDefinitions`/`Tags`, `DeletedText`, SmartType, Macros, SplitState — [S32](https://github.com/wildwinter/screenplay-tools/blob/main/tests/TestFDX-FI.fdx).
  - Version 3 (a file dated 2018): full FD preamble incl. `DisplayBoards`, `ListItems` (Beat Board / Story Map, introduced in FD10), `UnanchoredScriptNotes`, `CharacterNavigatorPreferences`, `AltCollection`, `Revisions`, `LockedPages`, `Cast`/`Actors` — [S01](https://github.com/4444J99/narratological-algorithmic-lenses/blob/main/specs/11-el-series/drafts/2018-03-12-tale-of-el.fdx).
  - Version 4 adds `TagData`, `Characters` (traits/inclusivity), `Images`, `TagsNavigatorPreferences` — [S11](https://github.com/extratone/corn/blob/main/docs/Children%20of%20The%20Corn%2030.fdx), [S28](https://github.com/SimonGZ/jumpcut/blob/main/tests/fixtures/fdx-import/title-page-cast-page.fdx).
  - Version 5 adds `Writers`, `WriterMarkup`, `ScriptNotes` (replacing `UnanchoredScriptNotes`), `Outlines`, `SceneNavigatorPreferences` — [S31](https://github.com/wildwinter/screenplay-tools/blob/main/tests/TestFDX-FD.fdx), [cullvox/bluemetal Maginvox](https://github.com/cullvox/bluemetal/blob/master/Docs/Maginvox.fdx).
  - Version 6 adds `DocumentRef`/`XRef`, `Navigator`, `ScriptNavigatorPreferences` (replacing scene/character/tags navigator prefs), paragraph `id` UUIDs, outline paragraph types in the default element set (`Outline 1–4`, `Outline Body`, `Summary`, `Note`, `Sequence`) — all 10 V6 samples; every V6 file has internal timestamps from 2024–2026, e.g. [S17](https://github.com/regosen/Phil-Ochs-screenplay/blob/main/The%20War%20Is%20Over.fdx).
- The open-fdx-toolkit feature matrix (FD8–12) claims tags and locked pages are FD11+ and Beat/StoryMap FD10+, but marks many cells "?" and its "verified" FD11/12 file reports `Version="2"`, which in our samples is the Fade In value — [FDX_SPEC.md](https://github.com/sfingali/open-fdx-toolkit/blob/main/FDX_SPEC.md). Treat its version columns as low confidence.
- The open-fdx-toolkit spec also reports that FD writes ~7 attributes per run and ~8 per paragraph while Fade In writes only meaningful ones (853 KB vs 520 KB for the same script) — [FDX_SPEC.md](https://github.com/sfingali/open-fdx-toolkit/blob/main/FDX_SPEC.md). Our FD13 samples write far fewer body attributes than that, so FD13 seems to have moved to sparse output for body paragraphs.

### Inferences
- Version mapping (inference from dates and features, Beat Board = FD10 2017, character traits/tags-in-file = FD11, collaboration Writers = FD12, FD13 2023): 3 → FD10, 4 → FD11, 5 → FD12, 6 → FD13. Versions 1/2 probably cover FD8/FD9; converters (screenplain, lexington, fountain2fdx) write `Version="1"` and Fade In writes `2`, and FD opens both, so a minimal `Content` + `TitlePage` file is accepted by current FD.
- For export, the project currently writes `Version="5"` with only Content/TitlePage. That is plausible (FD opens sparse files) but an exporter that wants FD13 behaviour should target Version 6 and include at least `ElementSettings`, `PageLayout`, `HeaderAndFooter`, `SceneNumberOptions` and `MoresAndContinueds` so FD does not fall back to its own defaults.

### Gaps
- No FD8 or FD9 authored file with a confirmed product version was found under a permissive licence; the GPL repo [joejhansen/script-editor](https://github.com/joejhansen/script-editor) has an FD11/FD13 pair (`ConnieFD11.fdx`, `BlankFD13.fdx`) that would confirm the 4 → FD11 / 6 → FD13 mapping, but it was not downloaded (GPL).
- Whether FD12 opens Version 6 files, and what it drops, is unconfirmed (KB article blocked).

---

## Q3. How are revisions, locked pages, A/B pages, omitted scenes, scene numbers, dual dialogue, notes, tags and alternates encoded?

### Takeaway
Revisions, scene numbers, omitted scenes, dual dialogue and notes are visible in public files. Locked
pages, A/B pages, populated tags and alternates (Alt Dialogue) were **not** found populated in any
permissively licensed sample. Their containers exist (`LockedPages`, `TagDefinitions`/`Tags`,
`AltCollection`) but the child structure has to be learned from files we make ourselves with FD.

### Cited Findings
- **Revisions**: revised text is marked per run with `Text@RevisionID` = `Revision@ID` of a set in `<Revisions>`; `0` = original. Revision sets carry name, mark character (`*`), mark colour and page colour; `ActiveSet` picks the current set and `RevisionMode="Yes"` turns tracking on — [S28](https://github.com/SimonGZ/jumpcut/blob/main/tests/fixtures/fdx-import/title-page-cast-page.fdx), [S26](https://github.com/shivanshv45/Bluepages/blob/main/tests/fixtures/feature-draft-1.fdx) (synthetic, RevisionID 1–3 on runs). Default FD sets are named "Blue Rev. (mm/dd/yy)", "Pink Rev. …" through "2nd White/Blue/Pink …" with standard page colours — [S01](https://github.com/4444J99/narratological-algorithmic-lenses/blob/main/specs/11-el-series/drafts/2018-03-12-tale-of-el.fdx).
- **Deleted text under revision** is not kept as text; it leaves a marker `DeletedText/DeletedTextLocation Offset="1229" RevisionID="1"` so FD can draw a revision mark where text was removed — [S28](https://github.com/SimonGZ/jumpcut/blob/main/tests/fixtures/fdx-import/title-page-cast-page.fdx).
- Header/footer `DynamicLabel Type="Collated Revisions"` / `"Last Revised"` print the revision label on pages — inventory.
- **Scene numbers**: `Paragraph@Number` on scene headings, free text (`9A`, `18A`, `I-1-A`, `9.` seen); numbering scheme in `SceneNumberOptions@NumberScheme="1A"`; left/right display toggles there — [S26](https://github.com/shivanshv45/Bluepages/blob/main/tests/fixtures/feature-draft-1.fdx), inventory. Unnumbered scripts simply omit `Number`.
- **Omitted scenes**: a `Scene Heading` paragraph keeping its `Number`, with text `OMITTED` and `SceneProperties Length="0/8"` — [S26](https://github.com/shivanshv45/Bluepages/blob/main/tests/fixtures/feature-draft-1.fdx) (synthetic). The same OMITTED convention appears in [Fade In 5.0.15 output](https://github.com/phagmaier/slugline/blob/main/testdata/fdx/fade-in-5.0.15.fdx) and production files in [madbradsmith/EVOLUM](https://github.com/madbradsmith/EVOLUM) (GPL / no licence; found by search, not downloaded). FD can also keep the omitted scene's original content for later recall, per its help index ([KB search snippet](https://kb.finaldraft.com/hc/en-us/articles/27817054455828-How-do-I-lock-or-unlock-script-pages-and-use-the-locking-tools)), so there is likely an extra hidden-content structure (unknown).
- **Locked pages / A pages**: after Production > Lock Pages, overflow goes onto A pages (15A) instead of pushing later pages — [Final Draft KB (search snippet)](https://kb.finaldraft.com/hc/en-us/articles/27817054455828-How-do-I-lock-or-unlock-script-pages-and-use-the-locking-tools). In XML, `<LockedPages/>` is empty in every sample. Search hits for `PageNumber` attributes appeared in files with real revisions ([captioncast small.fdx](https://github.com/OregonShakespeareFestival/captioncast/blob/master/small.fdx), GPL; [ShootingChecklist EuroMechanical.fdx](https://github.com/michaelforrest/ShootingChecklist/blob/master/ShootingChecklist/Reference/EuroMechanical.fdx), no licence) — not inspected.
- **Dual dialogue**: a wrapper `Paragraph` (no `Type` from screenplain; FD writes the first cue's attributes) containing `<DualDialogue>` with the two speeches as `Character`/`Parenthetical`/`Dialogue` paragraphs in order — [S24](https://github.com/vilcans/screenplain/blob/master/tests/files/dual-dialogue.fdx); open-fdx-toolkit reports the outer paragraph as `Type="Character"` in a Fade In export and shows `ScriptNote` and `DualDialogue` side by side in one paragraph — [FDX_SPEC.md](https://github.com/sfingali/open-fdx-toolkit/blob/main/FDX_SPEC.md); lexington models `DualDialogue` as an optional child of Paragraph — [lexington fdx package](https://pkg.go.dev/github.com/LaPingvino/lexington/fdx). The second cue is the second `Character` inside the wrapper.
- **Script notes**: two forms. (a) Inline `<ScriptNote>` as a child of the anchoring paragraph with `Author`, `Color`, `DateTime`, `DateModified`, `Name`, `Type` and `Paragraph/Text` body — [FDX_SPEC.md](https://github.com/sfingali/open-fdx-toolkit/blob/main/FDX_SPEC.md), [storyboarder test.fdx (2013)](https://github.com/wonderunit/storyboarder/blob/master/test/fixtures/final-draft/test.fdx) (no licence detected; inspected only). (b) FD12/13 top-level `ScriptNotes/ScriptNote` with `Range="start,end"` offsets and `RefId`/`WriterID` — [S03](https://github.com/45inertia/Screenwriting-Project-Dissertation/blob/main/Documentation/market_research/final_draft/star_wars_test_screenplay.fdx). In S03 the `RefId` does not match any paragraph `id`, so anchoring is probably by `Range`.
- **Tags**: categories are in `TagData/TagCategories`; `TagDefinitions` and `Tags` exist but were empty everywhere — [S32](https://github.com/wildwinter/screenplay-tools/blob/main/tests/TestFDX-FI.fdx), [EricQXu/Overlay_AR](https://github.com/EricQXu/Overlay_AR/blob/HEAD/Overlay_AR.fdx). Third-party tools (e.g. Filmustage) export tagged FDX that FD shows in Tags mode — [Filmustage help](https://help.filmustage.com/en/articles/6523169-export-script-to-final-draft-fdx). Code that parses populated tags exists only in GPL/AGPL projects (Beat, OpenShotDesigner, GoRenku/studio), not read.
- **Alternates (Alt Dialogue, FD11+)**: container `<AltCollection/>` present in every FD file since Version 3, always empty in the samples — inventory. KB lists "Alt Dialogue" as a production feature stored in the file — [KB snippet](https://kb.finaldraft.com/hc/en-us/articles/45143952909972-When-I-try-to-open-an-FDX-in-Final-Draft-Writer-I-get-this-error-Unsupported-FDX-This-file-contains-features-that-prevent-it-from-opening-at-this-time).
- **Page breaks**: `StartsNewPage="Yes"` on the paragraph that begins the new page — [S10](https://github.com/dmongrel/fdx-mcp-server/blob/master/src/fdx/resources/NewFile.fdx), [lexington](https://pkg.go.dev/github.com/LaPingvino/lexington/fdx).
- **Acts / outline**: `New Act`, `End of Act`, `Act&Scene Break`, `Outline 1..4`, `Outline Body`, `Summary`, `Sequence`, `Chapter`, `End of Chapter` are ordinary paragraph types; whether they print is decided by `ElementSettings/FontSpec@Style` containing `HiddenText` (`Bold+HiddenText`, `Italic+HiddenText` in outline element settings) and `Outline@CanHide` — inventory, [S17](https://github.com/regosen/Phil-Ochs-screenplay/blob/main/The%20War%20Is%20Over.fdx).

### Inferences
- `HiddenText` is how FD stores non-printing outline/synopsis elements; this maps well onto the project's Section/Synopsis (not printed).
- For revisions, the cleanest model is "revision ID per run" plus a revision-set table plus deletion markers; Fountain cannot hold this, so it must live in a sidecar or be lost when saving to Fountain.

### Gaps
- Child structure of `LockedPages`, `AltCollection`, `TagDefinitions`, `Tags`, and any per-paragraph tag markers; how A pages and locked page numbers are stored; how an omitted scene's hidden content is stored. Best way to close: make our own FD13 test files (lock pages, add A page, omit a scene, add alt dialogue, tag a prop) and inventory them — permitted by the clean-room rule in `docs/spec.md` ("files we create ourselves").

---

## Q4. What do open-source projects and app docs say about the format?

### Takeaway
There is no official spec. The most detailed permissively licensed write-up is open-fdx-toolkit's
`FDX_SPEC.md` (MIT), which is useful but has errors against real files. Most permissive libraries (screenplain,
lexington, fountain2fdx, screenplay-tools, jumpcut) handle only `Content`, `TitlePage`, `DualDialogue` and run
styles. The richer parsers are GPL/AGPL.

### Cited Findings
- [sfingali/open-fdx-toolkit](https://github.com/sfingali/open-fdx-toolkit) (MIT): Python parser/builder for FDX, Fountain, Fade In (`.fadein` = ZIP holding OSF XML), PDF; states the format was reverse-engineered from files; its `FDX_SPEC.md` covers root, paragraph types, run styles, SceneProperties/Summary, dual dialogue, ScriptNote, TagCategory, Revisions and LockedPages with confidence markers — [README](https://github.com/sfingali/open-fdx-toolkit), [FDX_SPEC.md](https://github.com/sfingali/open-fdx-toolkit/blob/main/FDX_SPEC.md). Errors found against samples: element order (Content first in FD), `Number` on SceneProperties, `8/8` cap.
- [vilcans/screenplain](https://github.com/vilcans/screenplain) (MIT): Fountain → FDX writer; writes `Version="1"`, untyped wrapper paragraph for dual dialogue, `Style` runs only — [S24](https://github.com/vilcans/screenplain/blob/master/tests/files/dual-dialogue.fdx).
- [LaPingvino/lexington](https://pkg.go.dev/github.com/LaPingvino/lexington/fdx) (MIT, Go): models FinalDraft root (DocumentType, Template, Version), Content, TitlePage, Paragraph (Type, Alignment, StartsNewPage, DualDialogue) and Text (AdornmentStyle, Background, Color, Font, RevisionID, Size, Style).
- [wildwinter/screenplay-tools](https://github.com/wildwinter/screenplay-tools) (MIT): Fountain/FDX tools in C++, C#, JS, Python with FD- and Fade-In-written reference files (S31, S32).
- [SimonGZ/jumpcut](https://github.com/SimonGZ/jumpcut) (MIT, Rust): FDX → Fountain keeps title pages and extra front-matter pages (cast lists) as text with page breaks and a `Frontmatter-page-count`, and turns paragraph layout overrides into modifier notes so they survive — [docs/formatting-and-metadata.md](https://github.com/SimonGZ/jumpcut/blob/main/docs/formatting-and-metadata.md).
- [Guernsey-Creative/screenplay-js](https://github.com/Guernsey-Creative/screenplay-js) (MIT) and [jzucker2/schoonmaker](https://github.com/jzucker2/schoonmaker) (MIT, FDX diff) are cited as sources by open-fdx-toolkit — [FDX_SPEC.md](https://github.com/sfingali/open-fdx-toolkit/blob/main/FDX_SPEC.md); not read further.
- [surrealroad XPath gist](https://gist.github.com/surrealroad/effaa4f84d8ba53cecb6): XPath for paragraphs, Type, Number, SceneProperties Title/Page, Summary/Paragraph/Text and ScriptNote.
- GPL/AGPL projects noted only, not read: [rsdoiel/fdx](https://github.com/rsdoiel/fdx) (AGPL-3.0, Go, has a `LockedPages` type and 6 test files) — [pkg.go.dev](https://pkg.go.dev/github.com/rsdoiel/fdx); Beat (lmparppei/Beat, GPL; FDX import/export incl. tag definitions); Trelby (GPL); OpenShotDesigner (GPL; FDX tag tests); GoRenku/studio (AGPL; FDX parser with tag fixtures); phagmaier/slugline (GPL; Fade In fixture).
- Final Draft's own position: FDX is open XML readable in any text editor; specs are shared with partners; scheduling/budgeting tools import tagged data (scene headings, locations, times, characters, props, costumes) — [Cast & Crew interview](https://blog.castandcrew.com/final-draft-interview).
- Final Draft KB confirms FD can import Fountain-based files — [KB](https://kb.finaldraft.com/hc/en-us/articles/15575076862228-Can-Final-Draft-import-a-file-written-in-a-Fountain-based-screenwriting-program).

### Inferences
- Since FD itself accepts sparse Version-1 files from screenplain etc., a minimal exporter is "valid"; the effort is in round-tripping what FD users put in the preamble.
- Fade In output (Version 2, sparse attributes, Number on Paragraph) is a good second dialect to test.

### Gaps
- Highland, WriterDuet, Fade In and Afterwriting docs on FDX import fidelity were not reached in this pass (search budget); Fade In's manual would be the best non-GPL source on how it maps tags and revisions.
- The [ArchiveTeam Final Draft page](http://fileformats.archiveteam.org/wiki/Final_Draft) could not be fetched (connection refused); search snippets only.

---

## Q5. Which sample FDX files are publicly downloadable, with URLs and licences?

### Takeaway
32 files were downloaded (all MIT except one Unlicense, one CC0, one Apache-2.0). 16 are genuine FD desktop
output (Versions 3–6), 1 Fade In, 9 converter output (Version 1), 3 minimal/unknown producer, 3 generated or
synthetic. The full catalogue with producer, size, licence and notes is in
`_research-media/storyteller/fdx-online/sources.md`.

### Cited Findings
- FD-authored, MIT unless noted: [01 tale-of-el (V3)](https://github.com/4444J99/narratological-algorithmic-lenses/blob/main/specs/11-el-series/drafts/2018-03-12-tale-of-el.fdx), [02 Untitled Screenplay (V6)](https://github.com/45inertia/Screenwriting-Project-Dissertation/blob/main/Documentation/market_research/final_draft/Untitled%20Screenplay.fdx), [03 star_wars_test (V6)](https://github.com/45inertia/Screenwriting-Project-Dissertation/blob/main/Documentation/market_research/final_draft/star_wars_test_screenplay.fdx), [07 A Tale of Existing (V5)](https://github.com/cullvox/bluemetal/blob/master/Docs/A%20Tale%20of%20Existing%20in%20the%20Modern%20World..fdx), [08 Maginvox (V5)](https://github.com/cullvox/bluemetal/blob/master/Docs/Maginvox.fdx), [09 Grog The Caveman (V6)](https://github.com/dmongrel/fdx-mcp-server/blob/master/examples/Grog%20The%20Caveman.fdx), [10 NewFile (V6)](https://github.com/dmongrel/fdx-mcp-server/blob/master/src/fdx/resources/NewFile.fdx), [11 Children of The Corn 30 (V4, Unlicense)](https://github.com/extratone/corn/blob/main/docs/Children%20of%20The%20Corn%2030.fdx), [14 testmanuscript (V6)](https://github.com/leechseed/_setsunadev/blob/main/.finaldraftfiles/testmanuscript.fdx), [15 outliers (V6)](https://github.com/leechseed/_setsunadev/blob/main/OUTLIERS_FINAL_DRAFT/outliers.fdx), [17 The War Is Over (V6)](https://github.com/regosen/Phil-Ochs-screenplay/blob/main/The%20War%20Is%20Over.fdx), [18 AotE_V01 (V6, CC0)](https://github.com/samuelflueckiger-spec/CAS-AICP/blob/main/Module%206/Script%20%28WIP%29/AotE_V01_20260515.fdx), [19 Don't threaten me (V6)](https://github.com/trevoralpert/Vertical-Video-Comedy-Sketch--.fdx-pdf_generator-/blob/main/Don%27t%20threaten%20me.fdx), [20 template (V6)](https://github.com/trevoralpert/Vertical-Video-Comedy-Sketch--.fdx-pdf_generator-/blob/main/old%20code/old%20code/template.fdx), [28 title-page-cast-page (V4)](https://github.com/SimonGZ/jumpcut/blob/main/tests/fixtures/fdx-import/title-page-cast-page.fdx), [31 TestFDX-FD (V5)](https://github.com/wildwinter/screenplay-tools/blob/main/tests/TestFDX-FD.fdx).
- Fade In: [32 TestFDX-FI (V2)](https://github.com/wildwinter/screenplay-tools/blob/main/tests/TestFDX-FI.fdx).
- Converter output (V1): [04](https://github.com/LaPingvino/lexington/blob/master/fdx/example.fdx), [05](https://github.com/LaPingvino/lexington/blob/master/fdx/example_inline_formatting.fdx) lexington; [06](https://github.com/briday1/fountain2fdx/blob/main/examples/example.fdx) fountain2fdx; [12](https://github.com/hughhowey/neo/blob/main/scripts/fixtures/screenplain.fdx), [21 Big-Fish](https://github.com/vilcans/screenplain/blob/master/examples/Big-Fish.fdx), [22 Brick-and-Steel](https://github.com/vilcans/screenplain/blob/master/examples/Brick-and-Steel.fdx), [23 The-Last-Birthday-Card](https://github.com/vilcans/screenplain/blob/master/examples/The-Last-Birthday-Card.fdx), [24 dual-dialogue](https://github.com/vilcans/screenplain/blob/master/tests/files/dual-dialogue.fdx), [25 simple](https://github.com/vilcans/screenplain/blob/master/tests/files/simple.fdx) screenplain.
- Minimal/unknown (V4, Content + TitlePage only): [13 bigfish (scrambled order)](https://github.com/intrusive-memory/SwiftEchada/blob/main/Fixtures/bigfish.fdx), [29 big-fish-scene-numbers](https://github.com/SimonGZ/jumpcut/blob/main/tests/fixtures/corpus/public/big-fish-scene-numbers/source/source.fdx), [30 gumshoe (letter-scrambled text)](https://github.com/SimonGZ/jumpcut/blob/main/tests/fixtures/corpus/public/gumshoe/source/source.fdx).
- Generated/synthetic: [16 cold-eggs (Apache-2.0)](https://github.com/localgpt-app/localgpt/blob/main/crates/previs/samples/cold-eggs.fdx), [26 feature-draft-1](https://github.com/shivanshv45/Bluepages/blob/main/tests/fixtures/feature-draft-1.fdx), [27 small-draft-2](https://github.com/shivanshv45/Bluepages/blob/main/tests/fixtures/small-draft-2.fdx).
- Content caveat: Big Fish (John August), Brick & Steel, The Last Birthday Card, Star Wars dialogue and the Children of the Corn fan script are copyrighted works even in MIT repos; use locally for tests, do not ship as fixtures. S17's title page contains the author's contact details.
- Found but not downloaded (GPL or no licence): FD11/FD13 pair in [joejhansen/script-editor](https://github.com/joejhansen/script-editor) (GPL-3.0); [captioncast small.fdx](https://github.com/OregonShakespeareFestival/captioncast) (GPL-2.0, real revisions); [madbradsmith/EVOLUM](https://github.com/madbradsmith/EVOLUM) (~20 production shooting drafts, no licence); [ms-liz-zhang/Sketches](https://github.com/ms-liz-zhang/Sketches) (~40 FD sketches, no licence); [nathanctay/finaler-draft final-draft-13-reference.fdx](https://github.com/nathanctay/finaler-draft) (no licence); [wonderunit/storyboarder test.fdx](https://github.com/wonderunit/storyboarder/blob/master/test/fixtures/final-draft/test.fdx) (licence not detected).

### Inferences
- The permissive set covers settings, Beat Board, character traits, title pages, revision sets, scene numbers and dual dialogue well, but has no populated tags, locked pages, A pages, alternates or genuine FD-written omitted scenes. Those need self-made FD files.

### Gaps
- Final Draft's own sample scripts ship with the app (bundled; off-limits under the clean-room rule) and no official downloadable FDX samples were found.

---

## Q6. What would the current method-storyteller importer lose?

### Takeaway
The importer in `crates/core/src/fdx.rs` keeps paragraph type, bold/italic/underline, scene numbers, dual
dialogue, `StartsNewPage` and a heuristic title page, and drops everything else, as `docs/spec.md` §5 says.
Running it on the samples (scratch build of `mst convert`) also showed three real bugs: nested paragraphs
inside SceneProperties/ScriptNote get promoted into printed Action, unknown outline types become printed
Action, and empty paragraphs become blank forced-action lines.

### Cited Findings (evidence: local code + running `mst convert` on the downloaded samples)
- **Bug: nested paragraphs leak into the script.** `handle_open` starts a new paragraph for every `Paragraph` element anywhere under `FinalDraft/Content`, including `SceneProperties/Summary/Paragraph`, `SceneProperties/SceneArcBeats/CharacterArcBeat/Paragraph` and `ScriptNote/Paragraph`. On close, they become `Action` elements placed *before* their parent (fdx.rs lines 360–385, 425–464). Effects: [S11](https://github.com/extratone/corn/blob/main/docs/Children%20of%20The%20Corn%2030.fdx) gains 76 blank `!` action lines from empty arc-beat paragraphs; in [S16](https://github.com/localgpt-app/localgpt/blob/main/crates/previs/samples/cold-eggs.fdx) the script note "the radio should name the storm…" becomes printed action above the line it annotates; a real FD `Summary/Paragraph` would become printed action above its scene heading.
- **Bug: outline types print.** `kind_from_fdx` maps unknown types to Action, so `Outline 1`, `Summary`, `New Act`, `End of Act`, `General`, `Cast List`, `Sequence`, `Note`, `Chapter`… print as action — in [S17](https://github.com/regosen/Phil-Ochs-screenplay/blob/main/The%20War%20Is%20Over.fdx) "Flashback 1" (Outline 1) and its synopsis (Summary) print. FD's `Section`/`Synopsis` equivalents are `Outline N`/`Summary`; the `Section` and `Synopsis` types the exporter writes do not exist in FD's default element set and will be shown by FD as unknown elements.
- **Bug: empty FD13 duplicate paragraphs** (`<Paragraph Type="Outline 1" id=…/>`) are kept because `kind` is set, giving a blank `!` line ([S17](https://github.com/regosen/Phil-Ochs-screenplay/blob/main/The%20War%20Is%20Over.fdx), 18 blank lines).
- Lost on import (by design): all preamble sections (ElementSettings, PageLayout, HeaderAndFooter, MoresAndContinueds, SceneNumberOptions, SmartType, Macros, Revisions, DeletedText, LockedPages, AltCollection, TagData, Cast/Actors/CastList, Characters, ScriptNotes, Writers, ListItems/DisplayBoards (Beat Board), Images, Watermarking, SpellCheck lists, UI state, DocumentRef); SceneProperties (Length, Page, Title, Color, Summary, arc beats); paragraph `id`, `Label`, `Alignment` other than Center-on-Action, per-paragraph indents/spacing; run `Font`, `Size`, `Color`, `Background`, `RevisionID`, `AdornmentStyle` and style tokens `AllCaps`, `HiddenText`, `Strikeout`; `Picture` in script; title-page layout (alignment beyond left/center/right, tabstops, multi-page front matter collapsed into Contact/Title fields).
- Lost on export: the writer emits only `Content` and `TitlePage` at `Version="5"`, uses non-FD types `Section` (with a `Level` attribute FD does not use) and `Synopsis`, writes Fountain notes as literal `[[…]]` text instead of `ScriptNote`, and adds a private `StorytellerField` attribute on title-page paragraphs (fdx.rs lines 18, 111–113, 139, 190). FD will apply its own default element settings, margins, header, and MORE/CONT'D rules.
- Works correctly on samples: all 32 files parse without error (`mst info` on each); scene numbers like `27A`, dual dialogue in [S24](https://github.com/vilcans/screenplain/blob/master/tests/files/dual-dialogue.fdx), and inline styles in [S05](https://github.com/LaPingvino/lexington/blob/master/fdx/example_inline_formatting.fdx) survive.

### Inferences
- Minimum fixes before claiming FD import: (1) only treat `Paragraph` elements that are direct children of `Content` or `DualDialogue` as script elements; route `Summary` → Synopsis, `ScriptNote` → note, arc beats → keep/drop; (2) map `Outline N` → Section (level N), `Summary` → Synopsis, and treat any type whose ElementSettings FontSpec has `HiddenText` as non-printing; (3) skip paragraphs with no text and no children; (4) export Section/Synopsis as `Outline N`/`Summary` (or define them in ElementSettings).
- For lossless round-trip, store the untouched preamble (and unknown paragraph/run attributes) as an opaque blob keyed to the document, and re-emit it on FDX save; drop only offset-based items (`ScriptNotes@Range`, `IgnoredRanges`, `DeletedTextLocation`) when text changed, or recompute them.
- Revision IDs per run, scene Length/Page caches and scene colours are cheap to keep in the model and matter to production users.

### Gaps
- Behaviour of FD itself when opening our exported files (which defaults it applies, whether `StorytellerField`/`Level` attributes survive an FD save) is untested; needs a manual check in FD by a user who owns it.
