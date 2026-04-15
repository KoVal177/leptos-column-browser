# CSS Theming

`leptos-column-browser` ships a single stylesheet (`style/column-browser.css`)
and exposes all colours via CSS custom properties (variables). There is no
runtime style injection; you control every pixel with CSS.

## Linking the Stylesheet

In your `index.html`:

```html
<link rel="stylesheet" href="path/to/column-browser.css" />
```

With Trunk:

```html
<link data-trunk rel="css" href="../../node_modules/leptos-column-browser/style/column-browser.css" />
```

Or copy `style/column-browser.css` into your project and manage it with your
own bundler.

## CSS Variable Reference

All variables live on the `.lcb-root` element, which is the outermost `<div>`
rendered by `ColumnBrowser` / `BrowserView`. Override them on `:root`,
`.lcb-root`, or any wrapping element.

| Variable | Default (dark) | Description |
|---|---|---|
| `--lcb-bg` | `#1e1e2e` | Root container background |
| `--lcb-column-bg` | `#181825` | Per-column background |
| `--lcb-border` | `#313244` | Column divider colour |
| `--lcb-item-hover` | `#313244` | Item hover background |
| `--lcb-selected` | `#45475a` | Selected item background |
| `--lcb-text` | `#cdd6f4` | Primary text colour |
| `--lcb-text-muted` | `#6c7086` | Secondary / muted text colour |
| `--lcb-accent` | `#89b4fa` | Icon colour, focus ring, resize handle |

The defaults use the [Catppuccin Mocha](https://github.com/catppuccin/catppuccin)
palette.

## Dark Theme (default)

No extra work is needed — the default palette is dark.

## Light Theme Override

```css
.lcb-root {
    --lcb-bg:         #eff1f5;
    --lcb-column-bg:  #e6e9ef;
    --lcb-border:     #ccd0da;
    --lcb-item-hover: #dce0e8;
    --lcb-selected:   #bcc0cc;
    --lcb-text:       #4c4f69;
    --lcb-text-muted: #8c8fa1;
    --lcb-accent:     #1e66f5;
}
```

## Scoped Themes

Apply different themes to different instances by wrapping each `ColumnBrowser`
in a dedicated container:

```html
<div class="my-dark-nav">
  <!-- ColumnBrowser rendered here -->
</div>
<div class="my-light-nav">
  <!-- ColumnBrowser rendered here -->
</div>
```

```css
.my-dark-nav .lcb-root  { --lcb-bg: #1e1e2e; /* ... */ }
.my-light-nav .lcb-root { --lcb-bg: #eff1f5; /* ... */ }
```

## Column Width Variables

```css
.lcb-root {
    --lcb-col-default-width: 200px;   /* initial column width */
    --lcb-col-min-width:      80px;   /* minimum after drag-resize */
}
```

These can also be configured via the `ColumnSizeConfig` prop rather than CSS,
which is preferred for programmatic control.
