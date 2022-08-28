# Current Solution to Syntax:
Is subject to change based on discussion.

- [Current Solution to Syntax:](#current-solution-to-syntax)
  - [Proposal 1: New Format](#proposal-1-new-format)
  - [Proposal 2: Loader](#proposal-2-loader)

## Proposal 1: New Format

```lua
-- Reserved names for groups:
{
  [1] = "#FFF" or "rgb(10, 20, 30)", -- Color of the element
  italic = true, -- Italic text (syntax only)
  bold = true, -- Bolded text (syntax only)
  strikethrough = true, -- Line through text (syntax only)
}

-- theme1.lua
return {
  
  -- There will be two objects exported, `editor` and `syntax` each with different reserved words and loading styles, as one can also govern fonts.
  editor = {
    
    -- Normal, base color, would color all children the same as the parent
    background = "white",

    -- Demonstrates "child" group proposed syntax
    statusbar = {
      "#FFF" -- Base/Default color style
      background = "#BBB", -- Subgroup styles
      text = "#333",
    },

    -- Demonstrates alternate child syntax
    ["scrollbar.hover"] = "#FFF"
  }

  -- Treated completely differently internally, since we are defining syntax styles
  syntax = {

    -- Same child based definitions here
    literal = "#000",
    keyword = {
      "#00F",
      constant = {
        "#000"
        bold = true,
      }
      builtin = {
        "#00F",
        ["function"] = "#000",
      }
    },

  }

  -- This would still work
  ["syntax.operator"] = "#000"
  ["editor.cursor"] = "#eee"
}

-- theme2.lua
local styles = { syntax = {}, editor = {} }
styles.editor.background = {}
styles.editor.background.track = "#FFF"
styles.editor.background.progress = "#FFF"
styles.editor.background.button = "#FFF"

styles.syntax.literal = { "#000" }
styles.syntax.literal.boolean = "#000"
return styles;
```

## Proposal 2: Loader

We have a special loader function, that loads files in as themes and processes them in a special way, called `theme`. We will do away with calling the `color` function manually, and call it internally after you provide a string either on `object[1]` or on the `object` itself, and it would cascade down to every style referencing it.

### Implications:
  
- This will add another file to the core directory.
- This will require changes to how Docview colors tokens

### Proposed usage:

```lua
local theme = require "core.theme"
theme.load("colors.theme1")
```

We could add additional utilities like scheduling and dynamic loading, like we had with the currently unsupported `theme16` plugin.

## Proposal 3: Naming Conventions

Firstly, we'd remove the currently active keyword `keyword2` completely, and only have styles based on the base keyword. Some examples:
  - 
