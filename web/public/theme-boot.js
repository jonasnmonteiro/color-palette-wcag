// Applies the stored theme before the first paint, so the page never shows one
// theme and then corrects itself.
//
// It is a file rather than an inline script on purpose: an inline script needs
// either 'unsafe-inline' or a hash in the Content-Security-Policy, and a hash
// goes stale every time the script changes. A same-origin file passes under a
// plain script-src 'self'. It carries no type="module" and no defer, so it is
// still synchronous and still runs before the first paint.
(function () {
  var root = document.documentElement;

  // The default is whatever the document already declares, so a deployment can
  // ship a different one by setting data-theme on <html>, and this file does
  // not have to know about it.
  var fallback = root.getAttribute("data-theme") || "dark";

  var theme = fallback;
  try {
    var stored = localStorage.getItem("colorust-app-theme");
    if (stored) theme = stored;
  } catch (e) {
    // Private windows and blocked site data both land here, and the fallback
    // is already correct.
  }

  root.setAttribute("data-theme", theme);
})();
