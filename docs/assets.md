# Static assets and CSS

Files below `static/` copy byte-for-byte into the output tree in lexicographic
order. Symbolic links are ignored and assets may not overwrite generated pages.
No discovered asset is executed.

The v0.1 CSS entry point is `static/css/site.css`. Theme CSS is copied below
`static/css/` and site CSS takes precedence by linking it later in a layout.
mkpage requires neither Node nor npm; fingerprinting, minification, and
external build hooks are deliberately absent from the default workflow.
