# Current Solution to Syntax:
Is subject to change based on discussion.

- [Current Solution to Syntax:](#current-solution-to-syntax)
  - [Proposal 1: New Format](#proposal-1-new-format)
  - [Proposal 2: Loader](#proposal-2-loader)
  - [Proposal 3: Naming Conventions](#proposal-3-naming-conventions)

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
  
  - `rust.symbols["Option"]`: `literal` -> `keyword.builtin.type.logic`
  - `rust.symbols["use"]`: `keyword` -> `keyword.builtin.import`
  - `rust.symbols["u16"]`: `keyword2` -> `keyword.builtin.type.number`
  - `javascript.symbols["const"]`: `keyword` -> `keyword.builtin.declaration.variable`

### Reasoning:

- `literal` vs `keyword`
  - Literals are values that have reserved names for values in that language, like `true`, `false`, `undefined`, `null` or `None`. Keywords on the other hand, is any name for an operation, or declaration, whether the symbol was the thing being declared or the thing declaring. Keywords can be user defined, like in `const name = "hello"`, `const` and `name` are both keywords. The definition is being stretched to this degree since the existing highlighting system used originally in Lite used `keyword` and `keyword2` in this way.
  - `keyword.builtin.*` is any **language defined** reserved symbol. `keyword.*` is for **user defined** names or symbols.

### Proposed names:

- `keyword`
  - `keyword.builtin`
    - `keyword.builtin.import`
    - `keyword.builtin.type`
    - `keyword.builtin.declaration`
    - `keyword.builtin.operator`
    - `keyword.builtin.conditional`
    - `keyword.builtin.loop`
    - `keyword.builtin.return`
  - `keyword.property`
  - `keyword.variable`
    - `keyword.variable.const`
  - `keyword.param`
    - `keyword.param.self`
  - `keyword.type`
    - `keyword.type.struct`
    - `keyword.type.enum`
    - `keyword.type.interface`
    - `keyword.type.trait`
    - `keyword.type.class`
  - `keyword.logic`
- `operator`
  - `operator.unary`
  - `operator.binary`
  - `operator.tertiary`
- `literal`
  - `literal.boolean`
  - `literal.null`
- `number`
  - `number.prefix`
  - `number.suffix`

