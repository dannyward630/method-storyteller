# Screenplay page layout and pagination rules (Final Draft defaults and industry references)

Research date: 2026-10-16. Clean-room: everything below is paraphrased from public documentation, help-centre articles and format guides. No GPL source code was read. Tags: **v1** = build now, **later** = a later phase, **skip** = do not build.

Confidence key: **[FD-KB]** = stated in Final Draft's own knowledge base; **[3P]** = third-party guide; **[INF]** = my inference or arithmetic; **[UNVERIFIED]** = recalled from general knowledge, not confirmed by a source fetched in this session. Verify these against a real Final Draft export before you rely on them.

---

## Q1. Element geometry, page frame, lines per page, header and page number

### Takeaway
Final Draft's default US Letter screenplay frame is 1.0" top and bottom margins, with a left margin of 1.5" and a right margin of 1.0" (the text column runs 1.5"–7.5"). At 6 lines per inch that gives **54 lines per page** on the default "Normal" spacing. Final Draft's own KB states the 54 figure, so it should be the v1 default, not 55. FD's templates also put **2 blank lines before Scene Heading and Shot**, which contradicts the project spec's MVP choice of 1.

### Cited findings
- **Lines per page (FD):** Under Document > Page Layout > Options, FD offers four line-spacing modes: Loose = 49 lines/page, **Normal = 54**, Tight = 58, Very Tight = 64. A script of exactly 100 pages on Normal repaginates to about 106 (Loose), 95 (Tight) and 89 (Very Tight). FD strongly recommends Normal. **[FD-KB]** — [Final Draft KB: How many lines per page](https://kb.finaldraft.com/hc/en-us/articles/15575339757716-How-many-lines-per-page-does-Final-Draft-write-and-what-are-my-line-spacing-options)
- **Pages are not filled to the last line:** FD keeps certain paragraphs together to avoid widows and orphans. Its example is a character name pushed to the next page so it stays with its dialogue. As a result, real pages often hold fewer than the nominal 54 lines. **[FD-KB]** — [same article](https://kb.finaldraft.com/hc/en-us/articles/15575339757716-How-many-lines-per-page-does-Final-Draft-write-and-what-are-my-line-spacing-options)
- **Other ways FD changes the page count:** per-paragraph leading (Format > Leading), per-paragraph indents set by dragging ruler tab stops, global element indents (Format > Elements > Paragraph tab, set as distance from the **left edge of the page**), global line spacing, and the top/bottom "Text" and "Header/Footer" margins under Document > Page Layout > Margins. **[FD-KB]** — [Final Draft KB: Can Final Draft cheat the page count?](https://kb.finaldraft.com/hc/en-us/articles/15575261962260)
  - Implication: FD stores indents as absolute positions measured from the left edge of the page. FDX files carry them per element, and Fade In reads them on import. See Q7.
- **Two blank lines before Scene Heading and Shot:** All of FD's screenplay templates set this, and FD calls it generally standard. Users can drop it to 1 in Format > Elements > Paragraph > Space Before. **[FD-KB]** — [FD KB: Why are there two blank lines above a Scene Heading and a Shot?](https://kb.finaldraft.com/hc/en-us/articles/15575271650452-Why-are-there-two-blank-lines-above-a-Scene-Heading-and-a-Shot); this is confirmed by [FD KB: Should there be two blank lines between transitions and scenes?](https://kb.finaldraft.com/hc/en-us/articles/15575250899732-Should-there-be-two-blank-lines-between-transitions-and-scenes), which says the default Screenplay template already does this.
  - Disagreement: a Reddit user's FD setup had Scene Heading "Space Before" set to 1 ([r/Screenwriting](https://www.reddit.com/r/Screenwriting/comments/11hebu3/about_final_draft_line_spacing_and_formatting/)). Write Brothers (Movie Magic) treats 1 or 2 as a free choice ([screenplay.com Basic Screenplay Format](https://screenplay.com/pages/basic-screenplay-format)). Older or user-modified templates vary.
- **Page margins (FD blog):** US Letter. Left 1.5", right 1", top 1", bottom 1". Action runs from 1.5" to the 1.0" right margin. Character starts about 3.7" from the left edge. Parenthetical starts about 3.1", with a right margin of about 2.9" (so it ends near 5.6"). Dialogue starts about 2.5", with a right margin of about 2.5" (so it ends near 6.0"). Transition is right-aligned to the 1.0" right margin. Page number sits top right, aligned to the right margin, **0.5" from the top**. The title page and page 1 carry no number. Scene numbers sit at 0.75" from the left edge and 1" from the right edge, in shooting scripts only. **[FD blog, approximate]** — [Final Draft blog: What are the margins for a screenplay?](https://www.finaldraft.com/blog/what-are-the-margins-for-a-screenplay)
- **FD "How to format" page:** top margin 1". Bottom and right margins may vary by up to a quarter inch. Left margin about 1.5" to leave room for binding. Page number flush right about 0.5" from the top, followed by a period, and none on page 1. (MORE) is uppercase, in parentheses, and described as "centered". **[FD marketing page]** — [finaldraft.com/learn/how-to-format-a-screenplay](https://www.finaldraft.com/learn/how-to-format-a-screenplay/)
- **Movie Magic Screenwriter (Write Brothers) element table**, with indents measured from the 1.5" text margin. Scene heading: 0" left, 6" wide, 1 or 2 blank lines before. Action: 0", 6" wide, 1 before. Character: 2.3" (that is, 3.8" from the page edge), 1 before. Dialogue: 1.0" (2.5" from the page edge), 3.5" wide (ends at 6.0"), 0 before. Parenthetical: 1.5" (3.0" from the page edge), 2.5" wide (ends at 5.5"), 0 before. Shot: 0", 6" wide, 1 before. Transition: flush right at 6" (7.5" from the page edge), 1 before. Courier 12 gives 10 cpi and 6 lpi. Page numbers sit inside the 1" top margin, flush right, with a trailing period, and none on page 1. Footers appear only on production scripts. **[3P, another vendor]** — [screenplay.com Basic Screenplay Format (Chris Huntley)](https://screenplay.com/pages/basic-screenplay-format)
- **Highland / Quote-Unquote (John August) guide:** about **55 lines per page**, not counting the page number and the blank lines after it. Character at 3.7", parenthetical at 3.1", dialogue at 2.5" (all from the page edge). Right margin about 1" (0.5"–1.25"), ragged. Page number top right, 0.5" from the top. **[3P]** — [Quote-Unquote Apps: Standard Screenplay Format](https://blog.quoteunquoteapps.com/standard-screenplay-format-the-writers-guide/)
- **FD element defaults [UNVERIFIED]:** I recall these per-element values from FD default-template FDX `ElementSettings`, given as left/right indent from the page edge and space before. Scene Heading 1.50/7.50, 2 before. Action 1.50/7.50, 1. Character 3.50/7.25, 1. Parenthetical 3.00/5.50, 0. Dialogue 2.50/6.00, 0. Transition 5.50/7.10, right-aligned, 1. Shot 1.50/7.50, 2. General 1.50/7.50, 0. No fetched source in this session confirmed them. Verify by opening a fresh FD template's FDX. The XML is data, not code, so reading it is clean-room-safe.

### Inferences
- **[INF]** 11" − 1" − 1" = 9" of body, and 9" × 6 lpi = **54 lines**, which matches FD's "Normal". The other modes also fit a fixed 9" body with changed leading: 9"/49 ≈ 13.2 pt, 9"/58 ≈ 11.2 pt, 9"/64 ≈ 10.1 pt. So FD's body is bounded by the 1" top and bottom text margins, and the header (page number at 0.5") sits outside it. **v1:** body = 54 lines × 12 pt starting at 1.0", page number baseline in the 0.5" header band.
- **[INF]** The "55 lines" figure (Highland guide, and the project spec) probably comes from a slightly smaller bottom margin, or from counting differently. Fifty-five lines would need 9.17" of body. To match FD, use 54.
- **[INF]** The sources disagree on the Character indent: 3.5" (FD template, unverified), 3.7" (FD blog, Highland), 3.8" (Movie Magic). This changes no line counts, because cues rarely wrap. It only moves the cue horizontally. Dialogue width matters much more: 2.5"–6.0" (35 chars) is consistent across FD and MMS. Parenthetical width is 3.0"–5.5" (25 chars) in MMS and in the FD values I recall, but 3.1"–5.6" in the FD blog. Same width, shifted 0.1".
- **v1 element table** (from the page edge; chars at 10 cpi):
  - Scene Heading: 1.5"–7.5", 60 ch, **2 blank lines before** (preference: 1)
  - Action: 1.5"–7.5", 60 ch, 1 before
  - Character: 3.5" left (FD) or 3.7"; never wraps in practice; 1 before
  - Parenthetical: 3.0"–5.5", 25 ch, 0 before
  - Dialogue: 2.5"–6.0", 35 ch, 0 before
  - Transition: right-aligned to 7.1"–7.5" (FD template unverified at 7.1"; blog and MMS at 7.5"), 1 before
  - Shot: 1.5"–7.5", 60 ch, 2 before (FD KB)
  - General: 1.5"–7.5", 0 before **[UNVERIFIED]**
- Page number: `N.` flush right to 7.5", baseline 0.5" from the top. Page 1 and the title page are unnumbered. **v1.**

### Gaps
- I could not fetch an authoritative table of FD's default indents for every element. The FD blog gives approximate values only. The exact right indents for Transition and Character, and General's space-before, need checking against an FD-generated FDX/PDF.
- The exact header/footer margin values in FD's Page Layout (Header 0.5"? Footer 0.5"?) were not confirmed.
- The width of the Courier Final Draft font is discussed under Q7.

---

## Q2. Pagination and breaking rules

### Takeaway
FD's documented behaviour has five parts. It keeps a character cue with its dialogue. It breaks action and dialogue only at sentence ends: if the last sentence would straddle the break, the whole sentence moves. This is an on-by-default checkbox in Page Layout > Options. It marks broken dialogue with `(MORE)` and `CHARACTER (CONT'D)`. It optionally adds `(CONT'D)` when a character resumes after action in the same scene. It never leaves a scene heading orphaned. FD does not publish minimum-line counts. Those must be found from FD output.

### Cited findings
- **Sentence-boundary breaking (default on):** When the last sentence on a page, in action or dialogue, would be cut by the page break, FD starts that sentence at the top of the next page. The checkbox is in Document > Page Layout > Options. When it is unchecked, text fills to the end of the line and breaks mid-sentence. **[FD-KB]** — [FD KB: I don't want dialogue and action to break at the end of a sentence](https://kb.finaldraft.com/hc/en-us/articles/15575343691284-I-don-t-want-dialogue-and-action-to-break-at-the-end-of-a-sentence-at-the-bottom-of-the-page)
- **Dialogue breaks:** FD tries to keep a speech on one page. When it cannot, it prints the Bottom-of-Page text `(MORE)` just beneath the dialogue on the first page. At the top of the next page it repeats the character name with the Top-of-Next-Page text `(CONT'D)`. Both texts are editable, and each has its own on/off toggle. **[FD-KB]** — [FD KB: Mores and Continueds settings in Final Draft Writer](https://kb.finaldraft.com/hc/en-us/articles/44647566253588-Mores-and-Continueds-settings-in-Final-Draft-Writer)
- **Automatic Character Continueds:** When this is on, FD appends the same `(CONT'D)` text to a character cue whenever that character's dialogue within a scene was interrupted by a non-dialogue element (Action or General). It is **not** added when the speech carries across a scene boundary. If another character speaks in between, it does not apply. **[FD-KB]** — [same article](https://kb.finaldraft.com/hc/en-us/articles/44647566253588-Mores-and-Continueds-settings-in-Final-Draft-Writer)
- **Character never orphaned:** FD moves a cue to the next page rather than leaving it at the bottom without its dialogue. **[FD-KB]** — [FD KB: lines per page](https://kb.finaldraft.com/hc/en-us/articles/15575339757716-How-many-lines-per-page-does-Final-Draft-write-and-what-are-my-line-spacing-options)
- **Parenthetical at the bottom of the page:** T. J. Alex says FD's default **does** allow a parenthetical to be the last thing before the break, with the dialogue continuing on the next page. He calls this ugly but standard-tool behaviour, and notes that only one pair of authors explicitly call it an error. **[3P]** — [Talentville: Screenplay Page Break rules (T. J. Alex)](https://www.talentville.com/snippet/screenplay-page-break-rules). He does not say whether that parenthetical sits before `(MORE)` or begins a new speech segment.
- **Scene heading:** never alone at the bottom of a page. Either the following action moves up with it or the heading moves down. The one exception Alex allows is an establishing-shot heading that stands alone as its own scene. **[3P]** — [Talentville](https://www.talentville.com/snippet/screenplay-page-break-rules)
- **Action:** never breaks mid-sentence. FD handles this automatically. No `(MORE)` is used for action breaks. **[3P]** — [Talentville](https://www.talentville.com/snippet/screenplay-page-break-rules)
- **Transition:** should be the last thing at the bottom of a page, never the first at the top. **[3P]** — [Talentville](https://www.talentville.com/snippet/screenplay-page-break-rules)
- **(MORE) position:** Alex says it is indented like a Character cue ([Talentville](https://www.talentville.com/snippet/screenplay-page-break-rules)). FD's learn page says "centered" ([FD learn](https://www.finaldraft.com/learn/how-to-format-a-screenplay/)). A Hollywood Standard summary says to break at a sentence end and put (MORE) beneath the speech ([Studocu summary of The Hollywood Standard](https://www.studocu.com/en-us/document/the-quest-center/ap-us-history/the-hollywood-standard/66939922), snippet only). In practice the character indent looks "centred" over the dialogue, so the sources probably agree.
- **Forced breaks:** Fountain uses a line of three or more `=` characters for an explicit page break, for example at TV act breaks. **[spec]** — [Fountain syntax](https://fountain.io/syntax/)
- **Start each scene or act on a new page:** FD has a KB article on making each scene or act begin on a new page, which suggests a per-element "start new page" option. Only the article title was seen. — [FD KB section listing: "How can I have each scene or act begin on a new page?"](https://kb.finaldraft.com/hc/en-us/articles/15575341475988-How-can-I-have-each-scene-or-act-begin-on-a-new-page)

### Inferences
- **v1 rules, refined:**
  1. *Keep-with-next for scene headings:* a heading must be followed on the same page by at least 1 complete sentence-unit (or 2 lines) of the next element. Otherwise it moves. Exception (**later**): a heading that is the last element of its scene may sit at the bottom.
  2. *Cue + dialogue:* a cue (and its parenthetical) must be followed by at least one line of dialogue on the same page.
  3. *Dialogue split:* only at a sentence end (`.`, `?`, `!`, plus trailing closing quote or paren) or a paragraph or parenthetical boundary. The top part ends with `(MORE)` at the character indent, which costs a line on page N. The bottom part starts with `CUE (CONT'D)`, which costs a line on page N+1. FD's exact minimums are not published. Use 2 lines on each side as the industry convention until FD output shows otherwise.
  4. *Parenthetical before (MORE):* Alex says FD allows it. The current spec forbids it. For FD parity, allow a speech to break *after a parenthetical only if* at least one dialogue line follows it on page N. Treat "a parenthetical as the last line before (MORE)" as **unconfirmed** and test it against FD output.
  5. *Action split:* only at a sentence end. If even the first sentence does not fit, the paragraph moves whole. Minimum lines are unpublished; use 2 on each side.
  6. *Transition:* never first on a page. Pull the previous line or element down with it.
  7. *Dual dialogue:* no source found on how FD splits it. Keep it whole for **v1**.
  8. *Toggle* "break at sentence end" on/off in the page layout (**later**). Default on.
- Top-of-page blank lines are suppressed. This is universal practice and is implied by FD's top-of-body positioning, but I found no explicit source.

### Gaps
- FD's exact minimum lines before and after a dialogue or action split (1? 2?) are not documented anywhere I could reach.
- Dual-dialogue page-break behaviour in FD: no source.
- Whether FD counts the `(MORE)` line inside the 54-line body: presumably yes, but not documented.
- Whether automatic character CONT'D and page-break CONT'D can stack (`(CONT'D)` once vs twice): not documented.

---

## Q3. Scene numbers, revisions, A-pages, locked pages, OMITTED, CONTINUED

### Takeaway
Production features follow a common convention:
- Scene numbers print in both margins.
- Changed lines get an asterisk in the right margin.
- Page headers show the revision colour and date.
- Pages added after locking become A-pages (12A).
- Deleted scenes stay as "OMITTED".
- Scene-continued markers are optional: `(CONTINUED)` at the bottom, `CONTINUED:` at the top, with an optional `(n)` count.

All of these are **later** except the scene-number placement.

### Cited findings
- **Scene CONTINUEDs (FD):** recommended for production scripts. When a scene runs across a page break, FD prints `(CONTINUED)` at the bottom of the first page. At the top of the next page it prints `CONTINUED:` together with the scene number. An option appends a running count, `(2)`, `(3)` and so on, on each further continued page of the same scene. Each text is editable and toggleable. **[FD-KB]** — [FD KB: Mores and Continueds](https://kb.finaldraft.com/hc/en-us/articles/44647566253588-Mores-and-Continueds-settings-in-Final-Draft-Writer)
- **Scene number positions:** about 0.75" from the left edge, and 1" from the right edge (that is, at about 7.5"). **[FD blog]** — [FD blog margins](https://www.finaldraft.com/blog/what-are-the-margins-for-a-screenplay). They are not used in spec scripts ([same](https://www.finaldraft.com/blog/what-are-the-margins-for-a-screenplay)).
- **Revision marks:** FD's Revision Mode puts a mark, usually an asterisk, in the right margin beside revised text. Revised text may also be coloured or styled. **[FD-KB snippet]** — [FD KB: How do I use Revision Mode?](https://kb.finaldraft.com/hc/en-us/articles/27813588722708-How-do-I-use-Revision-Mode)
- **Movie Magic production conventions:** revision-mark position is set as a distance from the right edge of the page, in tenths of an inch. Revised page headers show colour and date (e.g. a "BLUE REVISION" line with a date). A-pages can be numbered with the letter after (12A) or before (A12). When locked A-pages become short they can be "flexed" and merged. A deleted locked scene heading becomes OMITTED; two consecutive omits are joined with "AND" and longer runs with "THRU". Top and bottom scene continueds carry scene numbers. The revision colour order is user-configurable. **[3P, another vendor's manual]** — [Movie Magic Screenwriter Production Bible (PDF)](http://support.screenplay.com/filestore/mmsw4/docs/ProductionBible.pdf)
- **General convention:** inserted pages become 12A, 12B. A scene inserted between 10A and 10B becomes 10AA. Changed lines get a right-margin asterisk. Cut scenes keep their number with "OMITTED". **[3P]** — [Filmustage: Script Revision Colors](https://filmustage.com/blog/script-revision-colors-markup-history/); [Storiara: How to number a film script](https://storiara.com/blog/how-to-number-a-film-script-for-production)
- **Fountain scene numbers:** `#…#` after the heading, allowing alphanumerics, dashes and periods (e.g. `#1A#`). **[spec]** — [Fountain syntax](https://fountain.io/syntax/)

### Inferences
- **v1:** scene numbers at 0.75" (left) and right-aligned near 7.5"–8.0" (right). The spec's 7.75" is within the range sources give. Turn them off by default in spec scripts.
- **later:** scene CONTINUED (it costs 1 line at the bottom and 1 at the top of each continued page, which changes the page count noticeably, so it must be a real layout feature and not an overlay). Also later: revision asterisks (column at about 7.75"–8.0"), revised-page header (`<COLOR> REVISION <date>` plus page number), locked pages with A-pages, A-scene numbering, OMITTED/AND/THRU.
- **skip for now:** revision colour paper simulation.

### Gaps
- FD's exact default revision-mark column and header format (for example, whether the date sits left or right) were not confirmed from FD docs.
- How FD numbers A-pages when one page grows by several pages (12A, 12B…), and how it marks "12-13" combined pages: FD documentation was not reached, only MMS.

---

## Q4. Title page

### Takeaway
The title page is unnumbered. The title is uppercase and centred about a third of the way down. "Written by" and the author(s) are centred below it. Contact and draft date go at the lower left, or contact at the lower right for stage plays.

### Cited findings
- Fountain's recommended layout centres Title, Credit, Author(s) and Source, and places Contact and Draft date at the lower left. An implicit page break follows the title page. **[spec]** — [Fountain syntax](https://fountain.io/syntax/)
- Title in all caps, centred, about ¼ of the way down the page (about 20–22 lines below the 1" top margin). Byline about 2 lines below the title. — [SoCreate on Medium](https://socreate.medium.com/examples-of-screenplay-title-pages-dfdfe8eb0522); [SoCreate](https://www.socreate.it/en/blogs/screenwriting/how-to-format-a-title-page-in-traditional-screenwriting)
- Title uppercase, then "Written by", then the writer's name directly below. **[3P]** — [Quote-Unquote: Title page essentials](https://blog.quoteunquoteapps.com/title-page-essentials-make-your-script-look-professional/)
- The title page is never numbered, and page 1 is the first page of the script. **[3P]** — [Dramatists Guild formatting guidelines (PDF)](https://www.dramatistsguild.com/sites/default/files/2020-01/General-SFI-Formatting-Guidelines-Complete.pdf); [screenplay.com](https://screenplay.com/pages/basic-screenplay-format)

### Inferences
- **v1:** Title (uppercase, centred) at line about 18–22 of the body. Credit 2 lines below. Author 1 line below that. Source 2 lines below. Contact block and draft date bottom-left, ending at the last body line. Copyright bottom-left; for stage plays, contact goes bottom-right. Notes: **later**.

### Gaps
- FD's exact title-page template line positions were not found in its documentation.

---

## Q5. A4 vs US Letter

### Takeaway
FD supports exactly two paper types, US Letter and A4, chosen in Layout > Paper Type, with editable top and bottom margins. I found no documentation of how FD changes margins or line counts for A4.

### Cited findings
- FD has two paper sizes, US Letter and A4, switchable in Layout > Paper Type. Top and bottom margins can be adjusted there. **[FD-KB]** — [FD KB: change paper size and layout](https://kb.finaldraft.com/hc/en-us/articles/44986753080724-How-do-I-change-the-paper-size-and-layout-of-my-file-in-Final-Draft-Writer)
- Quote-Unquote claims about 55 lines per page "regardless of paper size". **[3P]** — [Quote-Unquote search snippet](https://blog.quoteunquoteapps.com/standard-screenplay-format-the-writers-guide/) (stated in the snippet; not re-verified on the full page)

### Inferences
- **[INF]** A4 is 8.27" × 11.69". With the same 1" top and bottom margins, the body would be 9.69", about 58 lines. FD's per-page count on A4 therefore depends on whether its A4 template enlarges the bottom margin to keep about 54–55 lines. The "regardless of paper size" claim suggests tools do. A4 is 0.23" narrower, so to keep the 60-character action line, the left and right margins must shrink (for example 1.3" / 0.9"). Otherwise every line rewraps.
- **later:** A4 support. When it is built, calibrate against an FD A4 PDF.

### Gaps
- FD's A4 default margins and lines per page are undocumented in the sources I reached. BBC Writersroom pages list format examples (for example the [BBC Medium and Format page](https://www.bbc.co.uk/writers/resources/medium-and-format)), but I did not fetch the A4 measurements from them.

---

## Q6. TV formats (multi-cam, single-cam) and stage plays

### Takeaway
Multi-cam differs in five ways: double-spaced dialogue, ALL-CAPS action, underlined headings, entrances and exits, acts and often scenes starting on new pages, and lettered scenes. All of it doubles page count. Single-cam is ordinary screenplay format with act-break markers. Stage plays use centred cues and parenthesised directions, with each act and scene starting on a new page.

### Cited findings
- **Multi-cam (screenwriting.io):** slugs are often underlined, sometimes with the characters in the scene listed in parentheses below. All action is ALL CAPS. Character names are underlined on first appearance. Entrances and exits are underlined. Sound cues are set off as `SOUND: …` and underlined. Dialogue is often double-spaced. Parentheticals are common and may sit inline. Scenes are lettered (for example "ACT 1 SCENE B") and sometimes each starts a new page. The page header often shows act and scene under the page number. Each act (and the cold open and tag) starts on a new page, with the centred ALL-CAPS act title about ⅓ of the way down the page. Each act ends with a centred "END OF ACT …". The episode ends with an underlined, right-aligned "FADE OUT." **[3P]** — [screenwriting.io: How are multicamera TV scripts formatted?](https://screenwriting.io/how-are-multicamera-tv-scripts-formatted/)
- **Double-spaced dialogue** makes a multi-cam script about twice as long as a single-cam one. **[3P]** — [WGFoundation: TV Format Fundamentals: Multi-cam sitcoms](https://www.wgfoundation.org/blog/2021/10/12/multi-cam-sitcoms) (snippet)
- **Multi-cam scenes start on new pages; action in caps.** **[3P]** — [ScreenWeaver](https://www.screenweaver.ai/blog/how-many-pages-30-minute-script) (snippet; lower-quality source)
- **Stage play (Dramatists Guild):** 1" margins (plus 0.5" on the left if bound). Character name centred or at 3.5" from the left edge, ALL CAPS. Stage directions start three indents from the left and sit in parentheses. Text is single-spaced, with a double space between speeches and long directions. Each act and scene starts on a new page, centred: the act in caps with a Roman numeral, the scene in title case and underlined, a double space below. Pagination is continuous and may embed act and scene (for example `II-1-51`). The title page is unnumbered. Title and byline are centred about halfway down. Contact goes bottom right and copyright bottom left. The guide suggests a font such as 12 pt Times. **[3P]** — [Dramatists Guild Formatting Guidelines (PDF)](https://www.dramatistsguild.com/sites/default/files/2020-01/General-SFI-Formatting-Guidelines-Complete.pdf)
- **Fountain forced breaks** are intended for TV act breaks. — [Fountain syntax](https://fountain.io/syntax/)

### Inferences
- **later:** multi-cam template. It needs a per-element "line spacing = 2" for dialogue, uppercase-action styling, "new page before" on Act and scene headings, a New Act element with about 18 blank lines on top, an End-of-Act element, and scene letters. Paginate with the same engine and a per-element `line_spacing` parameter.
- **later:** single-cam TV. Add Act/Teaser/Tag elements ("TEASER", "ACT ONE", "END OF ACT ONE"), each act starting on a new page.
- **skip (v1):** stage play.

### Gaps
- I did not reach FD's own multi-cam or single-cam template measurements. Network-specific variants are numerous.

---

## Q8. Comparison with docs/spec.md §5 (page model)

### Takeaway
The spec agrees on the page size, font metrics, left-edge indents for action, dialogue and parenthetical, page-number placement, and MORE/CONT'D. Three points contradict FD's documented defaults: 55 lines (FD says 54), 1 blank line before scene headings (FD templates use 2, and also 2 before Shot), and a ban on parentheticals before a break (a third party says FD allows them). Several spec rules have no FD documentation and need FD-output verification. The rule on minimum lines on each side of a split is one of them.

### Cited findings and status, by spec line
- **US Letter, Courier 12 pt, 10 cpi, 6 lpi:** **CONFIRMS.** — [screenplay.com](https://screenplay.com/pages/basic-screenplay-format). Refinement: FD's Courier Final Draft is slightly wider than 10 cpi — [Fade In KB](https://www.fadeinpro.com/kb/content/3/29/en/why-is-my-script-a-different-length-than-in-final-draft.html).
- **Body starts 1.0" from the top:** **CONFIRMS** — [FD blog](https://www.finaldraft.com/blog/what-are-the-margins-for-a-screenplay)
- **55 lines per page (configurable):** **CONTRADICTS** FD's Normal = 54 — [FD KB](https://kb.finaldraft.com/hc/en-us/articles/15575339757716-How-many-lines-per-page-does-Final-Draft-write-and-what-are-my-line-spacing-options). It matches Highland's ~55 — [Quote-Unquote](https://blog.quoteunquoteapps.com/standard-screenplay-format-the-writers-guide/). → Change the default to **54**, and add presets Loose 49 / Tight 58 / Very Tight 64 (**later**).
- **Page number `2.` top right, 0.5" from the top, ending at 7.5"; page 1 unnumbered:** **CONFIRMS** — [FD blog](https://www.finaldraft.com/blog/what-are-the-margins-for-a-screenplay); [FD learn](https://www.finaldraft.com/learn/how-to-format-a-screenplay/)
- **Scene heading / Action at 1.5", 60 ch:** **CONFIRMS** — [FD blog](https://www.finaldraft.com/blog/what-are-the-margins-for-a-screenplay); [screenplay.com](https://screenplay.com/pages/basic-screenplay-format)
- **Character at 3.7":** **CONFIRMS** the FD blog and Highland values. It differs from MMS (3.8") and from the FD template value I recall (3.5", **[UNVERIFIED]**). There is no line-count impact.
- **Parenthetical at 3.1", 25 ch:** **CONFIRMS** the FD blog (3.1" to about 5.6"). MMS gives 3.0"–5.5". Same width.
- **Dialogue at 2.5", 35 ch:** **CONFIRMS** — [FD blog](https://www.finaldraft.com/blog/what-are-the-margins-for-a-screenplay); [screenplay.com](https://screenplay.com/pages/basic-screenplay-format)
- **Transition right-aligned to 7.5":** **CONFIRMS** the FD blog and MMS. The FD template value of about 7.1" is [UNVERIFIED].
- **Shot at 1.5" with 1 blank line before:** **CONTRADICTS.** FD templates use 2 before Shot — [FD KB](https://kb.finaldraft.com/hc/en-us/articles/15575271650452-Why-are-there-two-blank-lines-above-a-Scene-Heading-and-a-Shot)
- **Scene heading 1 blank line before ("FD's default 1 space before"):** **CONTRADICTS.** All FD screenplay templates use **2** — [FD KB 1](https://kb.finaldraft.com/hc/en-us/articles/15575271650452-Why-are-there-two-blank-lines-above-a-Scene-Heading-and-a-Shot), [FD KB 2](https://kb.finaldraft.com/hc/en-us/articles/15575250899732-Should-there-be-two-blank-lines-between-transitions-and-scenes). This is the largest single page-count driver. A script with about 120 scenes gains about 120 lines, which is a bit over 2 pages.
- **Scene numbers at 0.75" and 7.75":** **REFINES.** The FD blog says 0.75" left and "1" from the right edge" (that is, at about 7.5") — [FD blog](https://www.finaldraft.com/blog/what-are-the-margins-for-a-screenplay). Verify against FD PDF output.
- **Blank lines at the top of a page dropped:** consistent with practice. No explicit source found.
- **Rule 1 (scene heading needs at least 2 following lines):** **CONFIRMS** the principle — [Talentville](https://www.talentville.com/snippet/screenplay-page-break-rules). The exact count of 2 is not documented by FD.
- **Rule 2 (cue never alone):** **CONFIRMS** — [FD KB](https://kb.finaldraft.com/hc/en-us/articles/15575339757716-How-many-lines-per-page-does-Final-Draft-write-and-what-are-my-line-spacing-options)
- **Rule 3 (dialogue split at a sentence or paragraph, with (MORE) and CONT'D):** **CONFIRMS** sentence-boundary breaking and the texts — [FD KB sentence](https://kb.finaldraft.com/hc/en-us/articles/15575343691284-I-don-t-want-dialogue-and-action-to-break-at-the-end-of-a-sentence-at-the-bottom-of-the-page); [FD KB Mores](https://kb.finaldraft.com/hc/en-us/articles/44647566253588-Mores-and-Continueds-settings-in-Final-Draft-Writer). The "at least 2 lines on each side" part is **UNVERIFIED**. "Parenthetical never last before (MORE)" is **POSSIBLY CONTRADICTED**: T. J. Alex says FD's default leaves parentheticals at page bottoms — [Talentville](https://www.talentville.com/snippet/screenplay-page-break-rules).
- **Rule 4 (action splits between sentences):** **CONFIRMS** — [FD KB sentence](https://kb.finaldraft.com/hc/en-us/articles/15575343691284-I-don-t-want-dialogue-and-action-to-break-at-the-end-of-a-sentence-at-the-bottom-of-the-page). The 2-line minimum is **UNVERIFIED**.
- **Rule 5 (transition never first on a page):** **CONFIRMS** — [Talentville](https://www.talentville.com/snippet/screenplay-page-break-rules)
- **Rule 6 (dual dialogue never splits):** no source either way.
- **Rule 7 (`===` forces a break):** **CONFIRMS** — [Fountain](https://fountain.io/syntax/)
- **Missing from the spec:**
  - Automatic Character CONT'D (same speaker resumes after action in the same scene) — **v1** if FD parity matters, because it changes cue text but not line counts.
  - The "break at sentence end" toggle — **later**.
  - Scene CONTINUED top and bottom with the `(n)` counter — **later**, and it affects page count.
  - Line-spacing presets — **later**.
  - General element (1.5", 0 before) — **v1** **[UNVERIFIED]**.

### Inferences
- The highest-value spec changes for page-count parity, in order:
  1. Scene heading and Shot space-before = 2.
  2. Lines per page = 54.
  3. Calibrate characters per line against FD PDF output (the Courier Final Draft width question).
  4. Confirm the parenthetical-at-bottom and minimum-lines behaviour on real FD output.

### Gaps
- None of the FD minimum-line thresholds are published. The only way to settle them is to generate test FDX files, render them in a licensed copy of FD, and inspect the PDFs. The output is public behaviour, which is consistent with the spec's clean-room approach.
