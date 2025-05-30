# My VSCode extensions and settings recommendations, regardless of the programming language

## Settings

### Ensure confirmation when deleting files

In *Preferences: Open User Settings* set *confirmDelete* to *true*

**Purpose:**

It will save you and not once

<!-- markdownlint-disable-next-line MD036 -->
**But**

Don't check **Don't ask me again**, because it would reset the setting

---

### Fix EOF (End of file) on file save

In *Preferences: Open User Settings* set `files.insertFinalNewline` to `true`

<!-- markdownlint-disable-next-line MD036 -->
**Purpose:**

- linters may complain
- precommits settings requiring proper EOFs won't slow you down anymore

---

### Remove trailing whitespaces on file save

In *Preferences: Open User Settings* set `files.trimTrailingWhitespace` to `true`

<!-- markdownlint-disable-next-line MD036 -->
**Purpose:**

- linters may complain
- precommits settings requiring proper EOFs won't slow you down anymore

---

### Markdown Link Checker

**Enable:**

In *Preferences* set `markdown.validate` to `true`

#### Exlude links from checking

In *Preferences: Open Workspace Settings* add links to ignore

---

### New terminal from other directory

**Purpose:**

In the case of [this-website-source-code](https://github.com/phantie/personal-website-source-code), when developing, I always go directly to the *./misc* directory to start development, run the local server, etc. This saves me from having to *cd misc* every time I open the project.

**Enable:**

In *Preferences: Open Workspace Settings* set `terminal.integrated.cwd` to relative path from which the terminal will be started.

Like *./misc* for [this-website-source-code](https://github.com/phantie/personal-website-source-code)

---

### Add paths to exclude from search

**Purpose:**

For example, I have a directory called `.common_links` in the root of my monorepository, which contains symlinks to commonly used files and directories. If I perform a search, the results will be duplicated: one in the original location and another via the symlink.

The goal is to prevent duplication by excluding the directories you want to prevent the search in.

**Enable:**

In *Preferences: Open Workspace Settings* in `search.exclude` add paths to ignore.

In my case I add `.common_links` to the list

---

## Extensions

### Conventional commits

**Link:**
[vivaxy.vscode-conventional-commits](https://marketplace.visualstudio.com/items?itemName=vivaxy.vscode-conventional-commits)

**Purpose:**

- For commit standardization

---

### Quick navigation between open repository and `GitHub, Bitbucket, Gitlab, VisualStudio.com`

**Link:**
[ziyasal.vscode-open-in-github](https://marketplace.visualstudio.com/items?itemName=ziyasal.vscode-open-in-github)

**Github repository example commands:**

- `Open in GitHub`
- `Copy GitHub link to clipboard`
- `Open Pull Request`

> also available through Explorer or shortcut keyboard commands

---

### markdownlint

**Link:**
[davidanson.vscode-markdownlint](https://marketplace.visualstudio.com/items?itemName=DavidAnson.vscode-markdownlint)

**Purpose:**

- .md validation
- .md formatting
- Warnings-ignoring configuration options
- Rich configuration (not yet figured out the configuration files usage)
- Etc.

<!-- markdownlint-disable-next-line MD036 -->
**Tune**

- In *Preferences: Open Workspace Settings* set *markdownlint.run* to *onSave*

<!-- markdownlint-disable-next-line MD036 -->
**Tips**

To turn off warning for a specific line use the example below with the appropriate warning code

```md
<!-- markdownlint-disable-next-line MD036 -->
**Warned line**
```

**Comment**
It might be rather annoying until you figure out how to ignore the warnings or tune it.

---

### Markdown Enhanced Code Block

**Link:**
[KiranMachhewar.markdown-enhanced-code-block](https://marketplace.visualstudio.com/items?itemName=KiranMachhewar.markdown-enhanced-code-block)

**Purpose:**

- Copy Code
  - Enables a copy button on the code block in markdown files to copy the code

- Run Code
  - Enables a run button on the code block in markdown files to run the code in the current active terminal (Opens new terminal if there is no existing)
    - Currently supported languages are
      - Bash/Shell Script (Default)
      - Etc (that I don't use)

- Etc

---

### Markdown Shortcuts

**Link:**
[mdickin.markdown-shortcuts](https://marketplace.visualstudio.com/items?itemName=mdickin.markdown-shortcuts)

**Purpose:**

Adds such shortcuts to preferences and key bindings as:

- md-shortcut.toggleBold Make **bold** ctrl+B
- md-shortcut.toggleItalic Make *italic* ctrl+I
- md-shortcut.toggleStrikethrough Make ~~strikethrough~~
- md-shortcut.toggleLink Make [a hyperlink](https://www.example.org) ctrl+L
- md-shortcut.toggleImage Make an image ![an image](/misc/public/favicon.ico) ctrl+shift+L
- md-shortcut.toggleCodeBlock Make ```a code block``` ctrl+M ctrl+C
- md-shortcut.toggleInlineCode Make `inline code` ctrl+M ctrl+I
- md-shortcut.toggleBullets Make * bullet point ctrl+M ctrl+B
- md-shortcut.toggleNumbers Make 1. numbered list ctrl+M ctrl+1
- md-shortcut.toggleCheckboxes Make - [ ] check list (Github flavored markdown) ctrl+M ctrl+X
- md-shortcut.toggleTitleH1 Toggle # H1 title
- md-shortcut.toggleTitleH2 Toggle ## H2 title
- md-shortcut.toggleTitleH3 Toggle ### H3 title
- md-shortcut.toggleTitleH4 Toggle #### H4 title
- md-shortcut.toggleTitleH5 Toggle ##### H5 title
- md-shortcut.toggleTitleH6 Toggle ###### H6 title
- md-shortcut.addTable Add Tabular values
- md-shortcut.addTableWithHeader

---

### Error Lens

**Link:**
[usernamehw.errorlens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens)

**Purpose:**

- Shows an error on the exact line it occurs
- More instant feedback compared to always needing to check the “Problems” in the terminal

---

### Symlink follow

**Link:**
[zaucy.symlink-follow](https://marketplace.visualstudio.com/items?itemName=zaucy.symlink-follow)
**Purpose:**

- Opens the file the symlink points to

**Recommendation:**

- Open Preferences (On MacOS Command+Shift+P)
- Select: “Preferences: Open User Settings”
- Find: symlink-follow.autoFollow
- Set it to: ✓

---

### Quick go to selected file path

**Link:**
[duXing.quick-go-to-selected-file-path](https://marketplace.visualstudio.com/items/?itemName=duXing.quick-go-to-selected-file-path)

**Purpose:**

- Quicker navigation through the repository

**Usage:**

1. Select a file path in the text editor or move cursor over the filename
2. Ctrl (or ⌘) + E: fill the text of selection to Quick Open panel
3. Press Enter to confirm

---

### Copy filename

**Link:**
[bradzacher.vscode-copy-filename](https://marketplace.visualstudio.com/items?itemName=bradzacher.vscode-copy-filename)

**Purpose:**

- Adds options to copy directory name / file name / file name without extensions by right-clicking on the file in the Explorer

**Exact commands added:**

- Copy Filename
- Copy Filename Without Extensions

---

### .puml viewer

**Link:**
[jebbs.plantuml](https://marketplace.visualstudio.com/items?itemName=jebbs.plantuml)

**Purpose:**

- For .puml preview

---

### .md viewer

**Link:**
[shd101wyy.markdown-preview-enhanced](https://marketplace.visualstudio.com/items?itemName=shd101wyy.markdown-preview-enhanced)

**Purpose:**

- For .md preview

---

### .ansi/log viewer

**Link:**
[iliazeus.vscode-ansi](https://marketplace.visualstudio.com/items?itemName=iliazeus.vscode-ansi)

**Purpose:**

- Provides a colorful preview for log files, matching console output conventions

---

### Copy Json Path as `a.b.c.d`

**Link:**
[Malo.copy-json-path](https://marketplace.visualstudio.com/items?itemName=Malo.copy-json-path)

**Purpose:**

- Optimizes workflow JSON files if you need to copy a path to a nested value as `a.b.c.d`

---

## Change Log

- 30/05/25 added `Malo.copy-json-path`
- 30/05/25 added `ziyasal.vscode-open-in-github`
