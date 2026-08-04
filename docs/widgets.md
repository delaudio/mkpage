# Semantic TUI widgets

Widgets are MiniJinja macros that emit semantic HTML; borders, monospace type,
and responsive styling are theme concerns. Copy
`examples/widgets/widgets.jinja` to the project's `layouts/widgets.jinja`, then
import `widgets.jinja` from a layout.

| Widget | HTML contract | Behaviour |
|---|---|---|
| Screen | page shell with optional header/footer | one per document; print normally |
| Pane | `section` with optional `h2` | nest only where heading hierarchy remains valid |
| Split | sibling regions in document order | collapses to stack on narrow screens |
| Stack | flow `div` | vertical only; no interactive behaviour |
| List | `ul`/`ol` and links | links work without JavaScript |
| Tree | nested `nav ul` | expanded hierarchy remains visible without JavaScript |
| Table | `table`, `caption`, `th` | data only, never layout |
| Tabs | `nav ul` of links followed by addressable panels | without JavaScript links jump to the target panel; enhancement may add tab semantics while preserving those links |
| Article | `article` | readable long-form content |
| StatusBar | `div role="status"` | advisory updates only; never sole location of essential content |
| KeyHints | labelled `nav ul` | hints describe shortcuts; pointer/touch alternatives remain visible |
| Dialog | `details`/`summary` with one content region | native keyboard, pointer, and touch operation; optional JavaScript may promote it to `dialog` |

Focus follows document order. Enhancements must preserve pointer, touch and
keyboard operation, honour reduced motion, and not depend on fixed dimensions.
StatusBar is a polite live region: use it only for short, changing advisory
messages, not for static page footer content.
Screen is the only page shell; Pane headings must not skip levels and the
optional label is reserved for panes without a visible title; Split and Stack
only contain sibling regions; Tree contains navigation lists; Table only
contains tabular data; Tabs are links until enhanced and must continue to
target real panel ids; Dialog content must not contain another Dialog. Every
widget prints as normal document flow and has no required motion; themes must
disable non-essential animation under `prefers-reduced-motion`.

Tabs start as plain links. Keyboard users tab through links in document order
and activate them with Enter; pointer and touch users activate the same links.
Without JavaScript the browser navigates to the linked panel by fragment id.
With JavaScript an enhancement layer may apply tab roles, arrow-key movement,
and selected-state styling, but it must keep the original link targets working.

```html
<nav class="mk-key-hints" aria-label="Keyboard shortcuts"><ul><li><a href="/">Home</a></li></ul></nav>
```

```html
<nav class="mk-tabs" aria-label="Sections">
  <ul>
    <li><a href="#panel-main">Main</a></li>
    <li><a href="#panel-notes">Notes</a></li>
  </ul>
</nav>
<article id="panel-main">Main panel</article>
<article id="panel-notes">Notes panel</article>
```

```jinja
{% from 'widgets.jinja' import screen, pane %}
{% call screen('Federico', 'ready') %}
  {% call pane('Projects') %}<ul><li><a href="/projects">mkpage</a></li></ul>{% endcall %}
{% endcall %}
```
