(function () {
  const VERSION = "1";
  const HELP_ID = "mkpage-keyboard-help";
  const ACTIVE_CLASS = "mkpage-enhanced-active";
  const ROUTE_SHORTCUT_TTL_MS = 350;

  const DEFAULT_SHORTCUTS = [
    { keys: ["g", "h"], href: "/", title: "Home" },
    { keys: ["g", "p"], href: "/projects/", title: "Projects" },
  ];

  const runtimeScript = document.currentScript || null;
  const config = parseConfig(runtimeScript);

  const containerSelector =
    "[data-mkpage-widget][data-mkpage-enhance='keyboard'], [data-mkpage-widget][data-mkpage-enhance='true']";
  const interactiveSelector =
    "a[href], button, [role='button'], [role='link'], [tabindex]:not([tabindex='-1'])";

  /** @type {{container: Element | null, item: Element | null}} */
  let state = { container: null, item: null };
  let routePrefix = null;
  let routeTimeoutId = null;

  if (!config.enabled) {
    return;
  }

  document.addEventListener("click", onPointerActivate, true);
  document.addEventListener("touchend", onPointerActivate, { passive: true });
  document.addEventListener("keydown", onKeydown, false);
  window.addEventListener("pagehide", teardownTransientState, { once: true });

  function parseConfig(scriptElement) {
    const rawShortcuts = scriptElement && scriptElement.dataset.mkpageRouteShortcuts;
    const routeShortcuts = normalizeRouteShortcuts(rawShortcuts);
    return {
      enabled: !(scriptElement && scriptElement.dataset.mkpageKeyboard === "off"),
      routeShortcuts: routeShortcuts.length ? routeShortcuts : DEFAULT_SHORTCUTS,
      helpTitle: scriptElement?.dataset.mkpageHelpTitle || "Keyboard shortcuts",
    };
  }

  function normalizeRouteShortcuts(raw) {
    if (!raw) {
      return [];
    }

    let parsed = null;

    try {
      parsed = JSON.parse(raw);
    } catch {
      return [];
    }

    if (Array.isArray(parsed)) {
      const flat = parsed
        .map(normalizeShortcutMap)
        .filter((item) => item !== null);
      return flat;
    }

    if (typeof parsed === "object" && parsed !== null) {
      return Object.entries(parsed)
        .map(([keyCombo, href]) => {
          if (typeof href !== "string") {
            return null;
          }
          return normalizeShortcutMap({ keys: keyCombo, href, title: keyCombo });
        })
        .filter((item) => item !== null);
    }

    return [];
  }

  function normalizeShortcutMap(raw) {
    if (!raw || typeof raw !== "object") {
      return null;
    }
    const keySource = typeof raw.keys === "string" ? raw.keys : raw.key;
    const href = typeof raw.href === "string" ? raw.href : raw.route;
    const title = typeof raw.title === "string" ? raw.title : keySource;

    if (!keySource || !href) {
      return null;
    }

    const keys = String(keySource)
      .trim()
      .toLowerCase()
      .split(/\s+/)
      .filter(Boolean);

    if (keys.length === 0) {
      return null;
    }

    return { keys, href: href.trim(), title: String(title).trim() };
  }

  function resolveContainer(node) {
    const container = node.closest(containerSelector);
    if (!container) {
      return null;
    }

    if (container.dataset.mkpageEnhance === "off") {
      return null;
    }

    if (container.closest('[data-mkpage-enhance="off"]')) {
      return null;
    }

    return container;
  }

  function onPointerActivate(event) {
    const target = event.target instanceof Element ? event.target : null;
    const item = target ? target.closest(interactiveSelector) : null;
    if (!item || !target) {
      return;
    }
    const container = resolveContainer(item);
    if (!container) {
      return;
    }
    if (isInEditable(target)) {
      return;
    }
    setActive(container, item);
  }

  function onKeydown(event) {
    if (event.defaultPrevented) {
      return;
    }
    const target = event.target;
    if (isInEditable(target) || isNavigationControl(target)) {
      return;
    }
    const key = event.key;
    if (!key) {
      return;
    }
    if (event.metaKey || event.ctrlKey || event.altKey) {
      return;
    }

    if (key === "?" && !event.shiftKey) {
      event.preventDefault();
      toggleHelp();
      return;
    }
    if (key === "/") {
      event.preventDefault();
      openCommandPalette();
      return;
    }
    if (key === "Escape") {
      event.preventDefault();
      teardownTransientState();
      return;
    }
    if (handleRouteShortcut(key)) {
      event.preventDefault();
      return;
    }
    if (key === "j" || key === "ArrowDown") {
      event.preventDefault();
      move(1);
      return;
    }
    if (key === "k" || key === "ArrowUp") {
      event.preventDefault();
      move(-1);
      return;
    }
    if (key === "ArrowRight") {
      event.preventDefault();
      move(1);
      return;
    }
    if (key === "ArrowLeft") {
      event.preventDefault();
      move(-1);
      return;
    }
    if (key === "Enter") {
      event.preventDefault();
      if (state.item && state.item instanceof Element) {
        state.item.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
      }
    }
  }

  function move(delta) {
    const container = state.container || firstKeyboardContainer();
    if (!container) {
      return;
    }

    const items = navigableItems(container);
    if (items.length === 0) {
      return;
    }

    const index = Math.max(0, items.indexOf(state.item));
    const nextIndex = ((index + delta) % items.length + items.length) % items.length;
    setActive(container, items[nextIndex]);
  }

  function setActive(container, item) {
    if (state.container) {
      clearActive(state.container);
    }
    state = { container, item };
    if (item instanceof Element) {
      item.classList.add(ACTIVE_CLASS);
      item.setAttribute("aria-current", "location");
      if (typeof item.focus === "function") {
        item.focus();
      }
    }
  }

  function clearActive(container) {
    if (!container) {
      return;
    }
    const active = container.querySelector(`.${ACTIVE_CLASS}`);
    if (active) {
      active.classList.remove(ACTIVE_CLASS);
      active.removeAttribute("aria-current");
    }
  }

  function handleRouteShortcut(key) {
    const lower = key.toLowerCase();
    if (!routePrefix) {
      if (lower === "g") {
        routePrefix = "g";
        routeTimeoutId = window.setTimeout(() => {
          routePrefix = null;
        }, ROUTE_SHORTCUT_TTL_MS);
        return true;
      }
      return false;
    }

    const sequence = [routePrefix, lower];
    const target = config.routeShortcuts.find((entry) => isRouteMatch(entry.keys, sequence));
    routePrefix = null;
    if (routeTimeoutId !== null) {
      window.clearTimeout(routeTimeoutId);
      routeTimeoutId = null;
    }

    if (!target) {
      return false;
    }
    window.location.assign(target.href);
    return true;
  }

  function isRouteMatch(shortcut, sequence) {
    if (!Array.isArray(shortcut) || shortcut.length !== sequence.length) {
      return false;
    }
    return shortcut.every((left, index) => left === sequence[index]);
  }

  function firstKeyboardContainer() {
    const first = document.querySelector(containerSelector);
    return first;
  }

  function navigableItems(container) {
    const all = Array.from(container.querySelectorAll(interactiveSelector));
    return all.filter((candidate) => {
      if (!candidate.offsetParent && candidate.getClientRects().length === 0) {
        return false;
      }
      const disabled =
        candidate.closest("[aria-disabled='true']") ||
        candidate.closest("[disabled]") ||
        candidate.closest("[data-mkpage-enhance='off']");
      return !disabled;
    });
  }

  function openCommandPalette() {
    const node = document.querySelector("[data-mkpage-command-palette]");
    if (node && "focus" in node) {
      node.focus();
      return;
    }

    const evt = new CustomEvent("mkpage:command-palette", {
      bubbles: true,
      cancelable: true,
      detail: {
        version: VERSION,
        routes: config.routeShortcuts,
      },
    });
    document.dispatchEvent(evt);
    if (evt.defaultPrevented) {
      return;
    }

    let help = document.getElementById(HELP_ID);
    if (!help) {
      help = document.createElement("aside");
      help.id = HELP_ID;
      help.setAttribute("role", "status");
      document.body.appendChild(help);
    }
    help.textContent = "Add a control with [data-mkpage-command-palette] to attach command palette input.";
    showTransientMessage(help);
  }

  function toggleHelp() {
    const existing = document.getElementById(HELP_ID);
    if (existing && existing.isConnected) {
      existing.remove();
      state.item = null;
      return;
    }

    const panel = document.createElement("dialog");
    panel.id = HELP_ID;
    panel.className = "mkpage-keyboard-help";
    panel.setAttribute("aria-label", config.helpTitle);
    panel.innerHTML = buildHelpMarkup();
    document.body.appendChild(panel);

    if (typeof panel.showModal === "function") {
      panel.showModal();
    }
  }

  function buildHelpMarkup() {
    const routeRows = config.routeShortcuts
      .map(
        (entry) =>
          `<li>${escapeHtml(entry.keys.join(" "))} → ${escapeHtml(entry.title)} (${escapeHtml(entry.href)})</li>`
      )
      .join("");
    return `<div><h2>${escapeHtml(config.helpTitle)}</h2><ul><li>j / ↓ move next</li><li>k / ↑ move previous</li><li>Enter activate</li><li>/ open command palette</li><li>Esc close overlays and clear selection</li>${routeRows ? `<li><strong>Route shortcuts</strong></li>${routeRows}` : ""}</ul></div>`;
  }

  function showTransientMessage(node) {
    node.style = "position:fixed;inset:auto 1rem 1rem; background:#111827; color:#e5e7eb; border:1px solid #334155; border-radius:0.5rem; padding:0.75rem; max-width:18rem;";
    node.setAttribute("role", "status");
    node.setAttribute("aria-live", "polite");
    node.hidden = false;
  }

  function teardownTransientState() {
    clearActive(state.container);
    state = { container: null, item: null };
    for (const details of document.querySelectorAll("details.mk-dialog[open]")) {
      details.removeAttribute("open");
    }
    const dialogs = document.querySelectorAll("dialog");
    for (const dialog of dialogs) {
      if (dialog.open) {
        dialog.close();
      }
    }

    const help = document.getElementById(HELP_ID);
    if (help) {
      help.remove();
    }
  }

  function isInEditable(target) {
    if (!(target instanceof Element)) {
      return false;
    }
    const tag = target.tagName;
    if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") {
      return true;
    }
    const role = target.getAttribute("role");
    if (role === "textbox" || role === "combobox" || role === "searchbox") {
      return true;
    }
    return target.isContentEditable;
  }

  function isNavigationControl(target) {
    if (!(target instanceof Element)) {
      return false;
    }
    return target.closest("a[href], button, [role='button'], [role='link'], input, textarea, select, [contenteditable='true'], details > summary") !== null;
  }

  function escapeHtml(value) {
    return String(value)
      .replaceAll("&", "&amp;")
      .replaceAll("<", "&lt;")
      .replaceAll(">", "&gt;");
  }
})();

