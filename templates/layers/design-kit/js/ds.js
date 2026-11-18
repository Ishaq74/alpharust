/*
 * ds.js — comportements du catalogue de composants, sans dépendance ni framework.
 * Même fichier pour Astro, Laravel (Vite) et Django/Wagtail (staticfiles).
 *
 *   tabs        data-slot="tabs" > tabs-list > tabs-trigger[data-value] ; tabs-content[data-value] ; data-default sur tabs
 *   dialog      dialog-trigger ouvre le <dialog data-slot="dialog-content"> du même dialog ; dialog-close le ferme
 *   switch      bascule aria-checked de switch-button
 *   accordion   data-type="single" : un seul <details> ouvert à la fois
 *   tooltip     relie aria-describedby du déclencheur à tooltip-content
 *   avatar      masque l'image en erreur (le repli reste visible)
 *
 * `window.DesignKit.init(racine)` ré-initialise une portion du DOM (navigation côté client).
 */
(function () {
  'use strict';

  var uid = 0;
  function id(prefix) {
    uid += 1;
    return 'ds-' + prefix + '-' + uid;
  }

  function slot(root, name) {
    return Array.prototype.slice.call(root.querySelectorAll('[data-slot="' + name + '"]'));
  }

  /* ---------- Onglets ---------- */
  function selectTab(tabs, value, focus) {
    var triggers = slot(tabs, 'tabs-trigger');
    var panels = slot(tabs, 'tabs-content');
    triggers.forEach(function (t) {
      var on = t.getAttribute('data-value') === value;
      t.setAttribute('aria-selected', on ? 'true' : 'false');
      t.tabIndex = on ? 0 : -1;
      if (on && focus) t.focus();
    });
    panels.forEach(function (p) {
      p.hidden = p.getAttribute('data-value') !== value;
    });
  }

  function initTabs(root) {
    slot(root, 'tabs').forEach(function (tabs) {
      if (tabs.dataset.dsReady) return;
      tabs.dataset.dsReady = '1';
      var triggers = slot(tabs, 'tabs-trigger');
      var panels = slot(tabs, 'tabs-content');
      var list = tabs.querySelector('[data-slot="tabs-list"]');
      if (list) list.setAttribute('role', 'tablist');
      triggers.forEach(function (t) {
        var v = t.getAttribute('data-value');
        var panel = panels.filter(function (p) {
          return p.getAttribute('data-value') === v;
        })[0];
        t.setAttribute('role', 'tab');
        if (!t.id) t.id = id('tab');
        if (panel) {
          if (!panel.id) panel.id = id('panel');
          t.setAttribute('aria-controls', panel.id);
          panel.setAttribute('role', 'tabpanel');
          panel.setAttribute('aria-labelledby', t.id);
        }
      });
      var initial = tabs.getAttribute('data-default') || (triggers[0] && triggers[0].getAttribute('data-value'));
      if (initial) selectTab(tabs, initial, false);
    });
  }

  document.addEventListener('click', function (e) {
    var t = e.target.closest && e.target.closest('[data-slot="tabs-trigger"]');
    if (t && !t.disabled) selectTab(t.closest('[data-slot="tabs"]'), t.getAttribute('data-value'), false);
  });

  document.addEventListener('keydown', function (e) {
    var t = e.target.closest && e.target.closest('[data-slot="tabs-trigger"]');
    if (!t) return;
    var tabs = t.closest('[data-slot="tabs"]');
    var all = slot(tabs, 'tabs-trigger').filter(function (x) { return !x.disabled; });
    var i = all.indexOf(t);
    var next = null;
    if (e.key === 'ArrowRight' || e.key === 'ArrowDown') next = all[(i + 1) % all.length];
    else if (e.key === 'ArrowLeft' || e.key === 'ArrowUp') next = all[(i - 1 + all.length) % all.length];
    else if (e.key === 'Home') next = all[0];
    else if (e.key === 'End') next = all[all.length - 1];
    if (next) {
      e.preventDefault();
      selectTab(tabs, next.getAttribute('data-value'), true);
    }
  });

  /* ---------- Boîte de dialogue ---------- */
  function dialogOf(el) {
    var wrap = el.closest('[data-slot="dialog"]');
    return wrap && wrap.querySelector('dialog[data-slot="dialog-content"]');
  }

  document.addEventListener('click', function (e) {
    var trigger = e.target.closest && e.target.closest('[data-slot="dialog-trigger"]');
    if (trigger) {
      var target = trigger.getAttribute('data-for');
      var dlg = target ? document.querySelector('#' + target + ' dialog[data-slot="dialog-content"]') : dialogOf(trigger);
      if (dlg && !dlg.open) dlg.showModal();
      return;
    }
    var close = e.target.closest && e.target.closest('[data-slot="dialog-close"]');
    if (close) {
      var d = close.closest('dialog');
      if (d) d.close();
      return;
    }
    // Clic sur le fond (::backdrop) : la cible est le <dialog> lui-même
    if (e.target.matches && e.target.matches('dialog[data-slot="dialog-content"]')) e.target.close();
  });

  function initDialogs(root) {
    slot(root, 'dialog-content').forEach(function (dlg) {
      if (dlg.dataset.dsReady) return;
      dlg.dataset.dsReady = '1';
      var title = dlg.querySelector('[data-slot="dialog-title"]');
      var desc = dlg.querySelector('[data-slot="dialog-description"]');
      if (title) {
        if (!title.id) title.id = id('dialog-title');
        dlg.setAttribute('aria-labelledby', title.id);
      }
      if (desc) {
        if (!desc.id) desc.id = id('dialog-desc');
        dlg.setAttribute('aria-describedby', desc.id);
      }
    });
    slot(root, 'dialog-trigger').forEach(function (t) {
      var el = t.hasAttribute('data-as-child') ? t.firstElementChild : t;
      if (el) el.setAttribute('aria-haspopup', 'dialog');
    });
  }

  /* ---------- Interrupteur ---------- */
  document.addEventListener('click', function (e) {
    var sw = e.target.closest && e.target.closest('[data-slot="switch-button"]');
    if (sw && !sw.disabled) sw.setAttribute('aria-checked', sw.getAttribute('aria-checked') === 'true' ? 'false' : 'true');
  });

  /* ---------- Accordéon ---------- */
  document.addEventListener('toggle', function (e) {
    var item = e.target;
    if (!item.matches || !item.matches('[data-slot="accordion-item"]') || !item.open) return;
    var root = item.closest('[data-slot="accordion"]');
    if (!root || root.getAttribute('data-type') === 'multiple') return;
    slot(root, 'accordion-item').forEach(function (other) {
      if (other !== item && other.open) other.open = false;
    });
  }, true);

  function initAccordions(root) {
    slot(root, 'accordion').forEach(function (acc) {
      var v = acc.getAttribute('data-default');
      if (!v || acc.dataset.dsReady) return;
      acc.dataset.dsReady = '1';
      slot(acc, 'accordion-item').forEach(function (item) {
        if (item.getAttribute('data-value') === v) item.open = true;
      });
    });
  }

  /* ---------- Info-bulle ---------- */
  function initTooltips(root) {
    slot(root, 'tooltip').forEach(function (tip) {
      var content = tip.querySelector('[data-slot="tooltip-content"]');
      var trigger = tip.querySelector('[data-slot="tooltip-trigger"]');
      if (!content || !trigger) return;
      if (!content.id) content.id = id('tooltip');
      var focusable = trigger.matches('a,button,input,select,textarea,[tabindex]') ? trigger : trigger.querySelector('a,button,input,select,textarea,[tabindex]');
      (focusable || trigger).setAttribute('aria-describedby', content.id);
    });
  }

  /* ---------- Avatar ---------- */
  document.addEventListener('error', function (e) {
    var img = e.target;
    if (img && img.matches && img.matches('[data-slot="avatar-image"]')) img.style.display = 'none';
  }, true);

  function init(root) {
    root = root || document;
    initTabs(root);
    initDialogs(root);
    initTooltips(root);
    initAccordions(root);
  }

  window.DesignKit = { init: init };
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', function () { init(document); });
  else init(document);
  document.addEventListener('astro:after-swap', function () { init(document); });
})();
