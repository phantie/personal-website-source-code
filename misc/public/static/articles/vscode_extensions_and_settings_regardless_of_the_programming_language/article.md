# My VSCode extensions and settings recommendations, regardless of the programming language

## Settings

### Ensure confirmation when deleting files

In *Preferences: Open User Settings* set *confirmDelete* to *true*

**Purpose:**

It will save you and not once

<!-- markdownlint-disable-next-line MD036 -->
**But**

Don't check **Don't ask me again**, because it would reset the setting

### Markdown Link Checker

**Enable:**  

In *Preferences* set *markdown.validate* to *true*

#### Exlude links from checking

In *Preferences: Open Workspace Settings* add links to ignore

---

### Start terminal from other dictory

**Purpose:**

In the case of [this-website-source-code](https://github.com/phantie/personal-website-source-code), when developing, I always go directly to the *./misc* directory to start development, run the local server, etc. This saves me from having to *cd misc* every time I open the project.

**Enable:**  

In *Preferences: Open Workspace Settings* set *terminal.integrated.cwd* to relative path from which the terminal will be started.

Like *./misc* for [this-website-source-code](https://github.com/phantie/personal-website-source-code)

---

## Extensions

### Conventional commits

**Link:**  
[vivaxy.vscode-conventional-commits](https://marketplace.visualstudio.com/items?itemName=vivaxy.vscode-conventional-commits)  

**Purpose:**  

- For commit standardization

---

### markdownlint

**Link:**  
[davidanson.vscode-markdownlint](https://marketplace.visualstudio.com/items?itemName=DavidAnson.vscode-markdownlint)

**Purpose:**  

- .md validation
- .md formatting
- Rich configuration (have not figured out yet)
- Etc

<!-- markdownlint-disable-next-line MD036 -->
**Tips**

To turn off warning for a specific line use the example below with the appropriate warning code

<!-- markdownlint-disable-next-line MD036 -->
**Tune**

- In *Preferences: Open Workspace Settings* set *markdownlint.run* to *onSave*

```md
<!-- markdownlint-disable-next-line MD036 -->
**Warned line**
```

**Comment**
Might be rather annoying until you figure out how to configure it

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
