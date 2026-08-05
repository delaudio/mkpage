# Progressive keyboard enhancements

mkpage can emit optional keyboard behavior as a separate JavaScript runtime.
This layer is off by default in fixtures unless enabled, and it never replaces
native links or required navigation.

## Enabling in a layout

If your layout should expose optional keyboard-first behavior, include the runtime:

```html
{% raw %}
{% if site.enhancements.keyboard %}
  <script
    type="module"
    src="/js/mkpage-keyboard-v1.js"
    data-mkpage-route-shortcuts='{"gh":"/","gp":"/projects/","gr":"/"}'
    data-mkpage-help-title="Keyboard shortcuts"
  ></script>
{% endif %}
{% endraw %}
```

`data-mkpage-route-shortcuts` accepts either a map of shortcut strings to
targets or a JSON array of objects:

```text
[{"keys":"g h","href":"/"},{"keys":"g p","href":"/projects/","title":"Projects"}]
```

Shortcut letters are matched only when captured in widget contexts.

## Widget opt-in and disablement

Only declared widgets receive enhancement behavior. Add these attributes to
interactive regions:

- `data-mkpage-widget="list"|"tree"|"tabs"|...`
- `data-mkpage-enhance="keyboard"`

Disable runtime on any subtree by setting `data-mkpage-enhance="off"`.

## Interaction map

Default keys handled by the runtime:

- `j` / `↓` move forward
- `k` / `↑` move backward
- `ArrowLeft` / `ArrowRight` move backward/forward
- `Enter` activate current item
- `Esc` close overlays and clear active state
- `?` toggles built-in contextual help derived from configured shortcuts
- `/` emits `mkpage:command-palette` event (site can handle with an existing
  search surface)

## Accessibility and no-JS behavior

The runtime never changes link semantics: links remain standard anchors and
browser navigation is preserved. If JavaScript is unavailable, pages continue to be
fully usable through native interactions.

Manual checks required in this scope:

- keyboard + screen-reader pass with route shortcuts and dialog/link widgets
- pointer/touch parity for same list and tab interactions
- mobile input focus behavior and `Esc` overlay close behavior

