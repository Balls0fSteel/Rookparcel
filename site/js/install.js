// The install guide's chooser (install.html): shows the steps for one system and one kind of
// file, one step at a time. Without JavaScript the page lists every path in full; nothing is
// stored or sent. The address's #fragment names the path, so other pages can link to one.
(function () {
  "use strict";
  var root = document.getElementById("guide");
  var pick = document.getElementById("pick");
  if (!root || !pick || !root.closest) return;

  var toc = document.getElementById("guide-toc");
  var systems = root.querySelectorAll(".guide-sys");
  var paths = root.querySelectorAll(".guide-path");
  var state = { sys: null, path: null, step: 0, all: false };

  function detect() {
    var ua = navigator.userAgent || "";
    if (/Windows NT/.test(ua)) return "windows";
    // an iPad asking for the desktop site also says Macintosh
    if (/Macintosh|Mac OS X/.test(ua) && !(navigator.maxTouchPoints > 1)) return "macos";
    if (/Linux|X11/.test(ua) && !/Android|CrOS/.test(ua)) return "linux";
    return null;
  }

  function pathEl(id) {
    var el = id ? document.getElementById(id) : null;
    return el && el.classList.contains("guide-path") ? el : null;
  }
  function sysOf(p) { return p.closest(".guide-sys").getAttribute("data-sys"); }
  function firstPath(sys) {
    var p = root.querySelector('.guide-sys[data-sys="' + sys + '"] .guide-path');
    return p ? p.id : null;
  }
  function stepsOf(p) { return p.querySelectorAll(".guide-steps > li"); }

  function choose(sys, path) {
    state.sys = sys || null;
    state.path = sys ? (path || firstPath(sys)) : null;
    state.step = 0;
  }

  // "#windows-zip" picks a path, "#macos" a system; anything else leaves the choice alone
  function fromHash() {
    var id = decodeURIComponent((location.hash || "").slice(1));
    var p = pathEl(id);
    if (p) { choose(sysOf(p), p.id); return true; }
    if (id && firstPath(id)) { choose(id, null); return true; }
    return false;
  }

  function button(text, cls) {
    var b = document.createElement("button");
    b.type = "button";
    b.className = cls;
    b.textContent = text;
    return b;
  }

  // Back / Step n of m / Next / Show all steps, under each path's list
  for (var i = 0; i < paths.length; i++) {
    var nav = document.createElement("div");
    nav.className = "guide-nav";
    var back = button("Back", "pick guide-back");
    var count = document.createElement("span");
    count.className = "guide-count";
    count.setAttribute("aria-live", "polite");
    var next = button("Next", "pick guide-next");
    var all = button("Show all steps", "guide-all");
    nav.appendChild(back);
    nav.appendChild(count);
    nav.appendChild(next);
    nav.appendChild(all);
    paths[i].appendChild(nav);
    var heads = paths[i].querySelectorAll(".guide-steps h4");
    for (var h = 0; h < heads.length; h++) heads[h].tabIndex = -1;
  }

  function render(focus) {
    var i, b;
    var buttons = pick.querySelectorAll("[data-pick-sys]");
    for (i = 0; i < buttons.length; i++) {
      b = buttons[i];
      b.setAttribute("aria-pressed", String((b.getAttribute("data-pick-sys") || null) === state.sys));
    }
    var rows = pick.querySelectorAll("[data-for]");
    for (i = 0; i < rows.length; i++) rows[i].hidden = rows[i].getAttribute("data-for") !== state.sys;
    buttons = pick.querySelectorAll("[data-pick-path]");
    for (i = 0; i < buttons.length; i++) {
      b = buttons[i];
      b.setAttribute("aria-pressed", String(b.getAttribute("data-pick-path") === state.path));
    }
    if (toc) toc.hidden = !!state.sys;
    for (i = 0; i < systems.length; i++) {
      systems[i].hidden = !!state.sys && systems[i].getAttribute("data-sys") !== state.sys;
    }
    for (i = 0; i < paths.length; i++) {
      var p = paths[i];
      var active = p.id === state.path;
      p.hidden = !!state.sys && !active;
      var steps = stepsOf(p);
      var single = active && !state.all;
      for (var s = 0; s < steps.length; s++) steps[s].hidden = single && s !== state.step;
      var nav = p.querySelector(".guide-nav");
      nav.hidden = !active;
      if (!active) continue;
      var last = steps.length - 1;
      nav.querySelector(".guide-back").hidden = !single;
      nav.querySelector(".guide-next").hidden = !single;
      nav.querySelector(".guide-count").hidden = !single;
      nav.querySelector(".guide-back").disabled = state.step === 0;
      nav.querySelector(".guide-next").disabled = state.step === last;
      nav.querySelector(".guide-count").textContent =
        state.step === last ? "Step " + (last + 1) + " of " + (last + 1) + ": done"
                            : "Step " + (state.step + 1) + " of " + (last + 1);
      nav.querySelector(".guide-all").textContent = state.all ? "One step at a time" : "Show all steps";
      if (focus && single) {
        var head = steps[state.step].querySelector("h4");
        if (head) head.focus();
      }
    }
  }

  function remember() {
    // keeps the address in step with the choice, so it can be copied; adds no history entry
    if (!window.history || !history.replaceState) return;
    history.replaceState(null, "", state.path ? "#" + state.path : location.pathname + location.search);
  }

  pick.addEventListener("click", function (e) {
    var b = e.target.closest("button");
    if (!b) return;
    if (b.hasAttribute("data-pick-sys")) choose(b.getAttribute("data-pick-sys"), null);
    else if (b.hasAttribute("data-pick-path")) choose(state.sys, b.getAttribute("data-pick-path"));
    else return;
    remember();
    render(false);
  });

  root.addEventListener("click", function (e) {
    var b = e.target.closest(".guide-nav button");
    if (!b) return;
    var p = pathEl(state.path);
    if (!p) return;
    var last = stepsOf(p).length - 1;
    if (b.classList.contains("guide-back")) state.step = Math.max(0, state.step - 1);
    else if (b.classList.contains("guide-next")) state.step = Math.min(last, state.step + 1);
    else if (b.classList.contains("guide-all")) state.all = !state.all;
    render(!b.classList.contains("guide-all"));
  });

  window.addEventListener("hashchange", function () {
    if (fromHash()) render(false);
  });

  if (!fromHash()) choose(detect(), null);
  pick.hidden = false;
  render(false);
})();
