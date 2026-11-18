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
 *   étendus     alert-dialog, sheet, collapsible, dropdown (+ sous-menus), popover, select (recherche, saisie au clavier),
 *               toggle / theme-toggle, radio-group, input-otp, input-group, dropzone, slider, carousel, sidebar, toast
 *               (window.__starwind__.toast, contrat de toast-manager.ts) ; mêmes événements que les composants Starwind.
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
  // Dialog et Sheet (dont le panneau mobile de Sidebar) partagent ce moteur : <dialog> natif + showModal.
  var DIALOG_ROOT = '[data-slot="dialog"], [data-slot="sheet"]';
  var DIALOG_EL = 'dialog[data-slot="dialog-content"], dialog[data-slot="sheet-content"]';

  function dialogOf(el) {
    var wrap = el.closest(DIALOG_ROOT);
    return wrap && wrap.querySelector(DIALOG_EL);
  }

  document.addEventListener('click', function (e) {
    var trigger = e.target.closest && e.target.closest('[data-slot="dialog-trigger"]');
    if (trigger) {
      var target = trigger.getAttribute('data-for');
      var host = target ? document.getElementById(target) : null;
      var dlg = host ? host.querySelector(DIALOG_EL) : dialogOf(trigger);
      if (dlg && !dlg.open) { dlg.showModal(); dlg.setAttribute('data-state', 'open'); }
      return;
    }
    var close = e.target.closest && e.target.closest('[data-slot="dialog-close"], [data-slot="sheet-close"]');
    if (close) {
      var d = close.closest('dialog') || dialogOf(close);
      if (d) { d.close(); d.setAttribute('data-state', 'closed'); }
      return;
    }
    // Clic sur le fond (::backdrop) : la cible est le <dialog> lui-même
    if (e.target.matches && e.target.matches(DIALOG_EL)) { e.target.close(); e.target.setAttribute('data-state', 'closed'); }
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

  /* =====================================================================================
   * Composants étendus (catalogue complet d'atomic en CSS pur). Mêmes événements que Starwind.
   * ===================================================================================== */
  function emit(el, name, detail) {
    el.dispatchEvent(new CustomEvent(name, { bubbles: true, detail: detail }));
  }

  function closest(el, sel) {
    return el && el.closest ? el.closest(sel) : null;
  }

  function focusables(root) {
    return Array.prototype.slice.call(root.querySelectorAll('a[href],button:not([disabled]),input:not([disabled]):not([type="hidden"]),select,textarea,[tabindex]:not([tabindex="-1"])'));
  }

  /* ---------- Boîte d'alerte ---------- */
  document.addEventListener('click', function (e) {
    var t = closest(e.target, '[data-slot="alert-dialog-trigger"]');
    if (t) {
      var target = t.getAttribute('data-dialog-for');
      var root = target ? document.getElementById(target) : closest(t, '[data-slot="alert-dialog"]');
      var dlg = root && root.querySelector('dialog[data-slot="alert-dialog-content"]');
      if (dlg && !dlg.open) {
        dlg.showModal();
        dlg.setAttribute('data-state', 'open');
        var cancel = dlg.querySelector('[data-slot="alert-dialog-cancel"]');
        if (cancel) (cancel.hasAttribute('data-as-child') ? cancel.firstElementChild || cancel : cancel).focus();
      }
      return;
    }
    var c = closest(e.target, '[data-slot="alert-dialog-action"], [data-slot="alert-dialog-cancel"]');
    if (c) {
      var d = closest(c, 'dialog');
      if (d) { d.close(); d.setAttribute('data-state', 'closed'); }
    }
  });

  function labelDialog(dlg, titleSlot, descSlot) {
    var title = dlg.querySelector('[data-slot="' + titleSlot + '"]');
    var desc = dlg.querySelector('[data-slot="' + descSlot + '"]');
    if (title) { if (!title.id) title.id = id('title'); dlg.setAttribute('aria-labelledby', title.id); }
    if (desc) { if (!desc.id) desc.id = id('desc'); dlg.setAttribute('aria-describedby', desc.id); }
  }

  function initDialogsExt(root) {
    slot(root, 'alert-dialog-content').forEach(function (d) { labelDialog(d, 'alert-dialog-title', 'alert-dialog-description'); });
    slot(root, 'sheet-content').forEach(function (d) { labelDialog(d, 'sheet-title', 'sheet-description'); });
  }

  /* ---------- Repliable ---------- */
  function setCollapsible(root, open) {
    root.setAttribute('data-state', open ? 'open' : 'closed');
    slot(root, 'collapsible-content').forEach(function (c) {
      if (closest(c, '[data-slot="collapsible"]') !== root) return;
      c.hidden = !open;
      c.setAttribute('data-state', open ? 'open' : 'closed');
    });
    slot(root, 'collapsible-trigger').forEach(function (t) {
      if (closest(t, '[data-slot="collapsible"]') !== root) return;
      var el = t.hasAttribute('data-as-child') ? t.firstElementChild || t : t;
      el.setAttribute('aria-expanded', open ? 'true' : 'false');
    });
  }

  function initCollapsibles(root) {
    slot(root, 'collapsible').forEach(function (c) {
      if (c.dataset.dsReady) return;
      c.dataset.dsReady = '1';
      var content = c.querySelector('[data-slot="collapsible-content"]');
      var trigger = c.querySelector('[data-slot="collapsible-trigger"]');
      if (content && trigger) {
        if (!content.id) content.id = id('collapsible');
        (trigger.hasAttribute('data-as-child') ? trigger.firstElementChild || trigger : trigger).setAttribute('aria-controls', content.id);
      }
      setCollapsible(c, c.getAttribute('data-state') === 'open');
    });
  }

  document.addEventListener('click', function (e) {
    var t = closest(e.target, '[data-slot="collapsible-trigger"]');
    if (!t) return;
    var c = closest(t, '[data-slot="collapsible"]');
    if (!c || c.hasAttribute('data-disabled')) return;
    setCollapsible(c, c.getAttribute('data-state') !== 'open');
  });

  /* ---------- Fenêtres flottantes : menu déroulant et popover ---------- */
  var FLOATING = { dropdown: ['dropdown-trigger', 'dropdown-content'], popover: ['popover-trigger', 'popover-content'] };

  function parts(root, kind) {
    var names = FLOATING[kind];
    var trigger = null, content = null;
    Array.prototype.forEach.call(root.children, function (ch) {
      if (ch.getAttribute('data-slot') === names[0]) trigger = ch;
      if (ch.getAttribute('data-slot') === names[1]) content = ch;
    });
    trigger = trigger || root.querySelector('[data-slot="' + names[0] + '"]');
    content = content || root.querySelector('[data-slot="' + names[1] + '"]');
    var el = trigger && trigger.hasAttribute('data-as-child') ? trigger.firstElementChild || trigger : trigger;
    return { trigger: el, content: content };
  }

  function menuItems(content) {
    return Array.prototype.filter.call(content.querySelectorAll('[data-slot="dropdown-item"]'), function (i) {
      return closest(i, '[data-slot="dropdown-content"]') === content && i.getAttribute('data-disabled') !== 'true';
    });
  }

  function setOpen(root, kind, open, focus) {
    var p = parts(root, kind);
    if (!p.content) return;
    if (open) {
      closeAllFloating(root);
      p.content.hidden = false;
      p.content.setAttribute('data-state', 'open');
      root.setAttribute('data-state', 'open');
      if (p.trigger) { p.trigger.setAttribute('aria-expanded', 'true'); p.trigger.setAttribute('data-state', 'open'); }
      if (focus) {
        if (kind === 'dropdown') { var items = menuItems(p.content); (focus === 'last' ? items[items.length - 1] : items[0] || p.content).focus(); }
        else (focusables(p.content)[0] || p.content).focus();
      }
    } else {
      p.content.hidden = true;
      p.content.setAttribute('data-state', 'closed');
      root.setAttribute('data-state', 'closed');
      if (p.trigger) { p.trigger.setAttribute('aria-expanded', 'false'); p.trigger.setAttribute('data-state', 'closed'); }
      slot(p.content, 'dropdown-content').forEach(function (sub) { sub.hidden = true; sub.setAttribute('data-state', 'closed'); });
      slot(p.content, 'dropdown-item').forEach(function (i) { if (i.hasAttribute('data-sub-trigger')) i.setAttribute('aria-expanded', 'false'); });
      if (focus && p.trigger) p.trigger.focus();
    }
  }

  function closeAllFloating(except) {
    ['dropdown', 'popover', 'select'].forEach(function (kind) {
      slot(document, kind).forEach(function (r) {
        if (r === except || r.contains(except)) return;
        if (r.getAttribute('data-state') === 'open') kind === 'select' ? setSelectOpen(r, false) : setOpen(r, kind, false);
      });
    });
  }

  function initFloating(root) {
    Object.keys(FLOATING).forEach(function (kind) {
      slot(root, kind).forEach(function (r) {
        if (r.dataset.dsReady) return;
        r.dataset.dsReady = '1';
        var p = parts(r, kind);
        if (p.content && p.trigger) {
          if (!p.content.id) p.content.id = id(kind);
          p.trigger.setAttribute('aria-controls', p.content.id);
          if (kind === 'dropdown' && !p.content.getAttribute('aria-labelledby')) {
            if (!p.trigger.id) p.trigger.id = id('trigger');
            p.content.setAttribute('aria-labelledby', p.trigger.id);
          }
        }
        if (r.getAttribute('data-default-open') === 'true') setOpen(r, kind, true);
        if (r.getAttribute('data-open-on-hover') === 'true') {
          var timer = null;
          var delay = parseInt(r.getAttribute('data-close-delay') || '200', 10);
          r.addEventListener('mouseenter', function () { clearTimeout(timer); setOpen(r, kind, true); });
          r.addEventListener('mouseleave', function () { timer = setTimeout(function () { setOpen(r, kind, false); }, delay); });
        }
      });
    });
  }

  document.addEventListener('click', function (e) {
    var kinds = ['dropdown', 'popover'];
    for (var k = 0; k < kinds.length; k++) {
      var kind = kinds[k];
      var t = closest(e.target, '[data-slot="' + FLOATING[kind][0] + '"]');
      if (t) {
        var r = closest(t, '[data-slot="' + kind + '"]');
        // à l'ouverture, le focus va au premier élément du menu (comme Starwind) ; Échap le rend au déclencheur
        var opening = r.getAttribute('data-state') !== 'open';
        setOpen(r, kind, opening, opening && kind === 'dropdown' ? 'first' : false);
        return;
      }
    }
    var item = closest(e.target, '[data-slot="dropdown-item"]');
    if (item && !item.hasAttribute('data-sub-trigger') && item.getAttribute('data-disabled') !== 'true') {
      var dd = closest(item, '[data-slot="dropdown"]');
      if (dd) setOpen(dd, 'dropdown', false, true);
      return;
    }
    if (item && item.hasAttribute('data-sub-trigger')) { openSub(item, true); return; }
    if (!closest(e.target, '[data-slot="dropdown-content"], [data-slot="popover-content"], [data-slot="select-content"]')) closeAllFloating(null);
  });

  function openSub(trigger, open) {
    var sub = closest(trigger, '[data-slot="dropdown-sub"]');
    var content = sub && sub.querySelector('[data-sub-content]');
    if (!content) return;
    content.hidden = !open;
    content.setAttribute('data-state', open ? 'open' : 'closed');
    trigger.setAttribute('aria-expanded', open ? 'true' : 'false');
    if (open) { var items = menuItems(content); if (items[0]) items[0].focus(); }
  }

  document.addEventListener('mouseover', function (e) {
    var t = closest(e.target, '[data-slot="dropdown-item"]');
    if (!t) return;
    var content = closest(t, '[data-slot="dropdown-content"]');
    slot(content, 'dropdown-item').forEach(function (i) {
      if (i.hasAttribute('data-sub-trigger') && i !== t && closest(i, '[data-slot="dropdown-content"]') === content) {
        var s = closest(i, '[data-slot="dropdown-sub"]').querySelector('[data-sub-content]');
        if (s) { s.hidden = true; i.setAttribute('aria-expanded', 'false'); }
      }
    });
    if (t.hasAttribute('data-sub-trigger')) {
      var s2 = closest(t, '[data-slot="dropdown-sub"]').querySelector('[data-sub-content]');
      if (s2) { s2.hidden = false; s2.setAttribute('data-state', 'open'); t.setAttribute('aria-expanded', 'true'); }
    }
  });

  document.addEventListener('keydown', function (e) {
    var trig = closest(e.target, '[data-slot="dropdown-trigger"]');
    if (trig && (e.key === 'ArrowDown' || e.key === 'ArrowUp')) {
      e.preventDefault();
      setOpen(closest(trig, '[data-slot="dropdown"]'), 'dropdown', true, e.key === 'ArrowUp' ? 'last' : 'first');
      return;
    }
    var content = closest(e.target, '[data-slot="dropdown-content"]');
    if (content) {
      var items = menuItems(content);
      var i = items.indexOf(e.target);
      var next = null;
      if (e.key === 'ArrowDown') next = items[(i + 1) % items.length];
      else if (e.key === 'ArrowUp') next = items[(i - 1 + items.length) % items.length];
      else if (e.key === 'Home') next = items[0];
      else if (e.key === 'End') next = items[items.length - 1];
      else if ((e.key === 'ArrowRight' || e.key === 'Enter' || e.key === ' ') && e.target.hasAttribute && e.target.hasAttribute('data-sub-trigger')) { e.preventDefault(); openSub(e.target, true); return; }
      else if (e.key === 'ArrowLeft' && content.hasAttribute('data-sub-content')) {
        e.preventDefault();
        var st = closest(content, '[data-slot="dropdown-sub"]').querySelector('[data-sub-trigger]');
        content.hidden = true; st.setAttribute('aria-expanded', 'false'); st.focus(); return;
      } else if (e.key === 'Enter' || e.key === ' ') { if (e.target.matches('[data-slot="dropdown-item"]')) { e.preventDefault(); e.target.click(); } return; }
      else if (e.key === 'Escape' || e.key === 'Tab') { var dd = closest(content, '[data-slot="dropdown"]'); setOpen(dd, 'dropdown', false, e.key === 'Escape'); return; }
      if (next) { e.preventDefault(); next.focus(); }
      return;
    }
    if (e.key === 'Escape') {
      var pc = closest(e.target, '[data-slot="popover"]');
      if (pc && pc.getAttribute('data-state') === 'open') { setOpen(pc, 'popover', false, true); return; }
      // Échap ferme aussi un menu ou un popover ouvert quand le focus est resté ailleurs
      ['dropdown', 'popover'].forEach(function (kind) {
        slot(document, kind).forEach(function (r) { if (r.getAttribute('data-state') === 'open') setOpen(r, kind, false, true); });
      });
    }
  });

  /* ---------- Liste déroulante riche (select) ---------- */
  function selectParts(r) {
    return {
      trigger: r.querySelector('[data-slot="select-trigger"]'),
      content: r.querySelector('[data-slot="select-content"]'),
      value: r.querySelector('[data-slot="select-value"]'),
      input: r.querySelector('[data-select-input]'),
      search: r.querySelector('[data-slot="select-search"]'),
      empty: r.querySelector('[data-slot="select-empty"]'),
    };
  }

  function selectItems(r, visibleOnly) {
    return slot(r, 'select-item').filter(function (i) { return i.getAttribute('data-disabled') !== 'true' && (!visibleOnly || !i.hidden); });
  }

  function chooseSelect(r, item, silent) {
    var p = selectParts(r);
    var value = item ? item.getAttribute('data-value') : '';
    slot(r, 'select-item').forEach(function (i) { i.setAttribute('aria-selected', i === item ? 'true' : 'false'); });
    r.setAttribute('data-value', value);
    if (p.input) p.input.value = value;
    var label = item ? item.textContent.trim() : '';
    if (p.value) {
      p.value.textContent = item ? label : p.value.getAttribute('data-placeholder') || '';
      if (item) p.value.removeAttribute('data-placeholder-shown'); else p.value.setAttribute('data-placeholder-shown', '');
    }
    if (!silent) emit(r, 'starwind-select:change', { value: value, selectId: r.id, label: label });
  }

  function setSelectOpen(r, open, focusLast) {
    var p = selectParts(r);
    if (!p.content) return;
    if (open) closeAllFloating(r);
    p.content.hidden = !open;
    p.content.setAttribute('data-state', open ? 'open' : 'closed');
    r.setAttribute('data-state', open ? 'open' : 'closed');
    if (p.trigger) { p.trigger.setAttribute('aria-expanded', open ? 'true' : 'false'); p.trigger.setAttribute('data-state', open ? 'open' : 'closed'); }
    if (open) {
      if (p.search) { p.search.value = ''; filterSelect(r, ''); p.search.focus(); return; }
      var items = selectItems(r, true);
      var cur = items.filter(function (i) { return i.getAttribute('aria-selected') === 'true'; })[0];
      (cur || (focusLast ? items[items.length - 1] : items[0]) || p.content).focus();
    } else if (p.trigger && r.contains(document.activeElement)) p.trigger.focus();
  }

  function filterSelect(r, q) {
    q = q.trim().toLowerCase();
    var any = false;
    slot(r, 'select-item').forEach(function (i) {
      var show = !q || i.textContent.toLowerCase().indexOf(q) !== -1;
      i.hidden = !show;
      any = any || show;
    });
    var p = selectParts(r);
    if (p.empty) p.empty.hidden = any;
  }

  function initSelects(root) {
    slot(root, 'select').forEach(function (r) {
      if (r.dataset.dsReady) return;
      r.dataset.dsReady = '1';
      var p = selectParts(r);
      if (p.content) {
        if (!p.content.id) p.content.id = id('listbox');
        if (p.trigger) p.trigger.setAttribute('aria-controls', p.content.id);
      }
      slot(r, 'select-item').forEach(function (i) { if (!i.id) i.id = id('option'); });
      if (p.value && !r.getAttribute('data-value')) p.value.setAttribute('data-placeholder-shown', '');
      var initial = r.getAttribute('data-value');
      var item = initial && selectItems(r, false).filter(function (i) { return i.getAttribute('data-value') === initial; })[0];
      if (item) chooseSelect(r, item, true);
    });
  }

  document.addEventListener('click', function (e) {
    var t = closest(e.target, '[data-slot="select-trigger"]');
    if (t && !t.disabled) { var r = closest(t, '[data-slot="select"]'); setSelectOpen(r, r.getAttribute('data-state') !== 'open'); return; }
    var it = closest(e.target, '[data-slot="select-item"]');
    if (it && it.getAttribute('data-disabled') !== 'true') { var r2 = closest(it, '[data-slot="select"]'); chooseSelect(r2, it); setSelectOpen(r2, false); }
  });

  document.addEventListener('input', function (e) {
    if (e.target.matches && e.target.matches('[data-slot="select-search"]')) filterSelect(closest(e.target, '[data-slot="select"]'), e.target.value);
  });

  var typeahead = { text: '', at: 0 };
  document.addEventListener('keydown', function (e) {
    var r = closest(e.target, '[data-slot="select"]');
    if (!r) return;
    var p = selectParts(r);
    var open = r.getAttribute('data-state') === 'open';
    if (e.target === p.trigger) {
      if (['ArrowDown', 'ArrowUp', 'Enter', ' '].indexOf(e.key) !== -1) { e.preventDefault(); setSelectOpen(r, true, e.key === 'ArrowUp'); }
      return;
    }
    if (!open) return;
    var items = selectItems(r, true);
    var i = items.indexOf(document.activeElement);
    var next = null;
    if (e.key === 'ArrowDown') next = items[Math.min(i + 1, items.length - 1)] || items[0];
    else if (e.key === 'ArrowUp') next = i <= 0 ? (e.target === p.search ? null : items[0]) : items[i - 1];
    else if (e.key === 'Home' && e.target !== p.search) next = items[0];
    else if (e.key === 'End' && e.target !== p.search) next = items[items.length - 1];
    else if (e.key === 'Escape' || e.key === 'Tab') { if (e.key === 'Escape') e.preventDefault(); setSelectOpen(r, false); return; }
    else if ((e.key === 'Enter' || (e.key === ' ' && e.target !== p.search)) && items[i]) { e.preventDefault(); chooseSelect(r, items[i]); setSelectOpen(r, false); return; }
    else if (e.key === 'Enter' && e.target === p.search && items.length) { e.preventDefault(); chooseSelect(r, items[0]); setSelectOpen(r, false); return; }
    else if (e.key.length === 1 && e.target !== p.search) {
      var now = Date.now();
      typeahead.text = (now - typeahead.at > 600 ? '' : typeahead.text) + e.key.toLowerCase();
      typeahead.at = now;
      next = items.filter(function (x) { return x.textContent.trim().toLowerCase().indexOf(typeahead.text) === 0; })[0];
    }
    if (next) { e.preventDefault(); next.focus(); }
  });

  /* ---------- Bascule et bascule de thème ---------- */
  function applyTheme(dark) {
    var html = document.documentElement;
    html.setAttribute('data-theme', dark ? 'dark' : 'light');
    html.classList.toggle('dark', dark);
    try { localStorage.setItem('theme', dark ? 'dark' : 'light'); } catch (err) { /* stockage indisponible */ }
  }

  function setToggle(t, on) {
    t.setAttribute('data-state', on ? 'on' : 'off');
    t.setAttribute('aria-pressed', on ? 'true' : 'false');
  }

  function initToggles(root) {
    var dark = document.documentElement.getAttribute('data-theme') === 'dark' || document.documentElement.classList.contains('dark');
    slot(root, 'theme-toggle').forEach(function (t) { setToggle(t, dark); });
  }

  document.addEventListener('click', function (e) {
    var t = closest(e.target, '[data-slot="toggle"], [data-slot="theme-toggle"]');
    if (!t || t.disabled) return;
    var on = t.getAttribute('data-state') !== 'on';
    setToggle(t, on);
    var group = t.getAttribute('data-sync-group');
    if (group) slot(document, 'toggle').forEach(function (o) { if (o !== t && o.getAttribute('data-sync-group') === group) setToggle(o, on); });
    emit(t, 'starwind-toggle:change', { pressed: on, toggleId: t.id, syncGroup: group || undefined });
    if (t.getAttribute('data-slot') === 'theme-toggle') {
      applyTheme(on);
      slot(document, 'theme-toggle').forEach(function (o) { setToggle(o, on); });
      document.dispatchEvent(new CustomEvent('theme:change', { detail: { theme: on ? 'dark' : 'light' } }));
    }
  });

  /* ---------- Boutons radio ---------- */
  function initRadioGroups(root) {
    slot(root, 'radio-group').forEach(function (g) {
      if (g.dataset.dsReady) return;
      g.dataset.dsReady = '1';
      var name = null;
      var items = slot(g, 'radio-group-item');
      items.forEach(function (i) { if (i.name) name = name || i.name; });
      name = name || g.getAttribute('name') || id('radio');
      var v = g.getAttribute('data-value');
      items.forEach(function (i) {
        if (!i.name) i.name = name;
        if (v && i.value === v) i.checked = true;
        var w = closest(i, '[data-slot="radio-group-item-wrapper"]');
        var label = w && w.parentElement && w.parentElement.tagName === 'LABEL';
        if (!i.id && !label && w && w.nextElementSibling && w.nextElementSibling.tagName === 'LABEL' && !w.nextElementSibling.htmlFor) {
          i.id = id('radio-item');
          w.nextElementSibling.htmlFor = i.id;
        }
      });
    });
  }

  document.addEventListener('change', function (e) {
    if (!e.target.matches || !e.target.matches('[data-slot="radio-group-item"]')) return;
    var g = closest(e.target, '[data-slot="radio-group"]');
    if (!g) return;
    g.setAttribute('data-value', e.target.value);
    emit(g, 'starwind:value-change', { value: e.target.value, radioGroupId: g.id || undefined });
  });

  /* ---------- Code à usage unique ---------- */
  function renderOtp(r, focused) {
    var input = r.querySelector('[data-otp-hidden-input]');
    var value = input ? input.value : '';
    var max = parseInt(r.getAttribute('data-max-length') || '6', 10);
    var slots = slot(r, 'input-otp-slot');
    var active = Math.min(value.length, max - 1);
    slots.forEach(function (s, idx) {
      var ch = s.querySelector('[data-otp-char]');
      if (ch) ch.textContent = value.charAt(idx);
      var on = focused && idx === active;
      s.setAttribute('data-active', on ? 'true' : 'false');
      var caret = s.querySelector('[data-otp-caret]');
      if (caret) caret.hidden = !(on && idx >= value.length);
    });
    r.setAttribute('data-value', value);
  }

  function initOtps(root) {
    slot(root, 'input-otp').forEach(function (r) {
      if (r.dataset.dsReady) return;
      r.dataset.dsReady = '1';
      var input = r.querySelector('[data-otp-hidden-input]');
      if (!input) return;
      var re = null;
      try { re = new RegExp(r.getAttribute('data-pattern') || '^\\d+$'); } catch (err) { re = /^\d+$/; }
      var max = parseInt(r.getAttribute('data-max-length') || '6', 10);
      r.addEventListener('click', function () { if (!input.disabled) input.focus(); });
      input.addEventListener('focus', function () { renderOtp(r, true); });
      input.addEventListener('blur', function () { renderOtp(r, false); });
      input.addEventListener('input', function () {
        var clean = input.value.split('').filter(function (c) { return re.test(c); }).join('').slice(0, max);
        input.value = clean;
        renderOtp(r, true);
        emit(r, 'starwind-input-otp:change', { value: clean, inputOtpId: r.id || undefined });
      });
      renderOtp(r, false);
    });
  }

  /* ---------- Groupe de saisie ---------- */
  document.addEventListener('click', function (e) {
    var addon = closest(e.target, '[data-slot="input-group-addon"]');
    if (!addon || closest(e.target, 'button, a, input, textarea, select')) return;
    var control = closest(addon, '[data-slot="input-group"]').querySelector('[data-slot="input-group-control"]');
    if (control) control.focus();
  });

  /* ---------- Zone de dépôt ---------- */
  function renderFiles(zone, files) {
    var list = zone.querySelector('[data-slot="dropzone-files-list"]');
    zone.setAttribute('data-has-files', files && files.length ? 'true' : 'false');
    if (!list) return;
    list.textContent = '';
    Array.prototype.forEach.call(files || [], function (f) {
      var item = document.createElement('div');
      item.textContent = f.name;
      list.appendChild(item);
    });
  }

  ['dragenter', 'dragover'].forEach(function (type) {
    document.addEventListener(type, function (e) {
      var z = closest(e.target, '[data-slot="dropzone"]');
      if (!z) return;
      e.preventDefault();
      z.setAttribute('data-drag-active', 'true');
    });
  });
  document.addEventListener('dragleave', function (e) {
    var z = closest(e.target, '[data-slot="dropzone"]');
    if (z && !z.contains(e.relatedTarget)) z.setAttribute('data-drag-active', 'false');
  });
  document.addEventListener('drop', function (e) {
    var z = closest(e.target, '[data-slot="dropzone"]');
    if (!z) return;
    e.preventDefault();
    z.setAttribute('data-drag-active', 'false');
    var input = z.querySelector('input[type="file"]');
    if (input && e.dataTransfer && e.dataTransfer.files.length) {
      try { input.files = e.dataTransfer.files; } catch (err) { /* navigateur ancien */ }
      input.dispatchEvent(new Event('change', { bubbles: true }));
    }
  });
  document.addEventListener('change', function (e) {
    if (!e.target.matches || !e.target.matches('[data-slot="dropzone"] input[type="file"]')) return;
    renderFiles(closest(e.target, '[data-slot="dropzone"]'), e.target.files);
  });
  document.addEventListener('keydown', function (e) {
    if (!e.target.matches || !e.target.matches('[data-slot="dropzone"]') || (e.key !== 'Enter' && e.key !== ' ')) return;
    e.preventDefault();
    var input = e.target.querySelector('input[type="file"]');
    if (input) input.click();
  });

  /* ---------- Curseur ---------- */
  function sliderState(s) {
    return {
      min: parseFloat(s.getAttribute('data-min') || '0'),
      max: parseFloat(s.getAttribute('data-max') || '100'),
      step: parseFloat(s.getAttribute('data-step') || '1'),
      large: parseFloat(s.getAttribute('data-large-step') || '10'),
      vertical: s.getAttribute('data-orientation') === 'vertical',
      thumbs: slot(s, 'slider-thumb'),
    };
  }

  function thumbValue(t) { return parseFloat(t.getAttribute('aria-valuenow') || '0'); }

  function setThumb(s, t, value, commit) {
    var st = sliderState(s);
    var idx = st.thumbs.indexOf(t);
    var lo = idx > 0 ? thumbValue(st.thumbs[idx - 1]) : st.min;
    var hi = idx < st.thumbs.length - 1 ? thumbValue(st.thumbs[idx + 1]) : st.max;
    value = Math.round((value - st.min) / st.step) * st.step + st.min;
    value = Math.min(hi, Math.max(lo, value));
    value = parseFloat(value.toFixed(6));
    t.setAttribute('aria-valuenow', String(value));
    var input = t.querySelector('input[type="range"]');
    if (input) input.value = String(value);
    var pct = function (v) { return ((v - st.min) / (st.max - st.min || 1)) * 100; };
    t.style[st.vertical ? 'bottom' : 'left'] = pct(value) + '%';
    var values = st.thumbs.map(thumbValue);
    var range = s.querySelector('[data-slot="slider-range"]');
    if (range) {
      var start = values.length > 1 ? pct(Math.min.apply(null, values)) : 0;
      var end = pct(Math.max.apply(null, values));
      if (st.vertical) { range.style.bottom = start + '%'; range.style.height = (end - start) + '%'; }
      else { range.style.left = start + '%'; range.style.width = (end - start) + '%'; }
    }
    var detail = { value: values.length > 1 ? values : values[0] };
    emit(s, commit ? 'slider-commit' : 'slider-change', detail);
  }

  document.addEventListener('pointerdown', function (e) {
    var control = closest(e.target, '[data-slot="slider-control"]');
    if (!control) return;
    var s = closest(control, '[data-slot="slider"]');
    if (s.hasAttribute('data-disabled')) return;
    var st = sliderState(s);
    var track = s.querySelector('[data-slot="slider-track"]');
    var valueAt = function (ev) {
      var rect = track.getBoundingClientRect();
      var ratio = st.vertical ? (rect.bottom - ev.clientY) / rect.height : (ev.clientX - rect.left) / rect.width;
      if (!st.vertical && getComputedStyle(s).direction === 'rtl') ratio = 1 - ratio;
      return st.min + Math.min(1, Math.max(0, ratio)) * (st.max - st.min);
    };
    var v = valueAt(e);
    var thumb = closest(e.target, '[data-slot="slider-thumb"]') || st.thumbs.reduce(function (best, t) {
      return Math.abs(thumbValue(t) - v) < Math.abs(thumbValue(best) - v) ? t : best;
    }, st.thumbs[0]);
    e.preventDefault();
    thumb.focus();
    setThumb(s, thumb, v, false);
    var move = function (ev) { setThumb(s, thumb, valueAt(ev), false); };
    var up = function () {
      document.removeEventListener('pointermove', move);
      document.removeEventListener('pointerup', up);
      setThumb(s, thumb, thumbValue(thumb), true);
    };
    document.addEventListener('pointermove', move);
    document.addEventListener('pointerup', up);
  });

  document.addEventListener('keydown', function (e) {
    if (!e.target.matches || !e.target.matches('[data-slot="slider-thumb"]')) return;
    var s = closest(e.target, '[data-slot="slider"]');
    var st = sliderState(s);
    var v = thumbValue(e.target);
    var rtl = getComputedStyle(s).direction === 'rtl' && !st.vertical;
    var step = e.shiftKey ? st.large : st.step;
    var map = {
      ArrowRight: rtl ? -step : step, ArrowUp: step, ArrowLeft: rtl ? step : -step, ArrowDown: -step,
      PageUp: st.large, PageDown: -st.large,
    };
    var next = null;
    if (e.key in map) next = v + map[e.key];
    else if (e.key === 'Home') next = st.min;
    else if (e.key === 'End') next = st.max;
    if (next === null) return;
    e.preventDefault();
    setThumb(s, e.target, next, true);
  });

  /* ---------- Carrousel ---------- */
  function carousel(el) {
    if (el.__dsCarousel) return el.__dsCarousel;
    var container = el.querySelector('[data-slot="carousel-container"]');
    var prev = el.querySelector('[data-slot="carousel-previous"]');
    var next = el.querySelector('[data-slot="carousel-next"]');
    var opts = {};
    try { opts = JSON.parse(el.getAttribute('data-opts') || '{}'); } catch (err) { opts = {}; }
    var vertical = el.getAttribute('data-axis') === 'y';
    var items = function () { return slot(el, 'carousel-item'); };
    var index = function () {
      var list = items();
      var pos = vertical ? container.scrollTop : Math.abs(container.scrollLeft);
      var size = list[0] ? (vertical ? list[0].offsetHeight : list[0].offsetWidth) : 1;
      return Math.round(pos / (size || 1));
    };
    var scrollTo = function (i) {
      var list = items();
      if (!list.length) return;
      if (opts.loop) i = (i + list.length) % list.length;
      i = Math.max(0, Math.min(list.length - 1, i));
      list[i].scrollIntoView({ behavior: 'smooth', block: 'nearest', inline: 'start' });
    };
    var update = function () {
      var list = items();
      var i = index();
      list.forEach(function (it, n) {
        it.setAttribute('aria-label', (n + 1) + ' / ' + list.length);
      });
      if (!opts.loop) {
        if (prev) prev.disabled = i <= 0;
        if (next) next.disabled = i >= list.length - 1;
      }
    };
    var api = {
      scrollPrev: function () { scrollTo(index() - 1); },
      scrollNext: function () { scrollTo(index() + 1); },
      scrollTo: scrollTo,
      selectedScrollSnap: index,
    };
    if (prev) prev.addEventListener('click', api.scrollPrev);
    if (next) next.addEventListener('click', api.scrollNext);
    if (container) {
      container.addEventListener('scroll', function () { window.requestAnimationFrame(update); }, { passive: true });
      container.tabIndex = container.tabIndex >= 0 ? container.tabIndex : 0;
    }
    el.addEventListener('keydown', function (e) {
      if (e.target.closest('input, textarea, select')) return;
      if (e.key === (vertical ? 'ArrowDown' : 'ArrowRight')) { e.preventDefault(); api.scrollNext(); }
      if (e.key === (vertical ? 'ArrowUp' : 'ArrowLeft')) { e.preventDefault(); api.scrollPrev(); }
    });
    if (opts.startIndex && container) {
      var list0 = items();
      if (list0[opts.startIndex]) container[vertical ? 'scrollTop' : 'scrollLeft'] = list0[opts.startIndex][vertical ? 'offsetTop' : 'offsetLeft'];
    }
    update();
    el.__dsCarousel = api;
    return api;
  }

  function initCarousels(root) {
    slot(root, 'carousel').forEach(function (el) { if (el.getAttribute('data-auto-init') !== 'false') carousel(el); });
  }

  /* ---------- Barre latérale ---------- */
  function toggleSidebar(provider) {
    if (!provider) return;
    var mobile = window.matchMedia('(max-width: 47.99rem)').matches;
    if (mobile) {
      var dlg = provider.querySelector('[data-sidebar-mobile] dialog[data-slot="sheet-content"]');
      if (dlg) { dlg.open ? dlg.close() : dlg.showModal(); provider.setAttribute('data-mobile-open', dlg.open ? 'true' : 'false'); }
      emit(provider, 'sidebar:mobile-change', { open: !!(dlg && dlg.open) });
      return;
    }
    var open = provider.getAttribute('data-state') !== 'expanded';
    provider.setAttribute('data-state', open ? 'expanded' : 'collapsed');
    slot(provider, 'sidebar').forEach(function (s) { s.setAttribute('data-state', open ? 'expanded' : 'collapsed'); });
    slot(provider, 'sidebar-trigger').forEach(function (t) { t.setAttribute('aria-expanded', open ? 'true' : 'false'); });
    emit(provider, 'sidebar:change', { state: open ? 'expanded' : 'collapsed', open: open });
  }

  document.addEventListener('click', function (e) {
    var t = closest(e.target, '[data-slot="sidebar-trigger"], [data-slot="sidebar-rail"]');
    if (t) toggleSidebar(closest(t, '[data-slot="sidebar-provider"]') || document.querySelector('[data-slot="sidebar-provider"]'));
  });

  document.addEventListener('keydown', function (e) {
    if (!(e.ctrlKey || e.metaKey) || e.altKey) return;
    slot(document, 'sidebar-provider').forEach(function (p) {
      if (e.key.toLowerCase() === (p.getAttribute('data-keyboard-shortcut') || 'b')) { e.preventDefault(); toggleSidebar(p); }
    });
  });

  function initSidebars(root) {
    slot(root, 'sidebar-provider').forEach(function (p) {
      var open = p.getAttribute('data-state') !== 'collapsed';
      slot(p, 'sidebar').forEach(function (s) { if (s.hasAttribute('data-collapsible')) s.setAttribute('data-state', open ? 'expanded' : 'collapsed'); });
      slot(p, 'sidebar-trigger').forEach(function (t) { t.setAttribute('aria-expanded', open ? 'true' : 'false'); });
    });
  }

  /* ---------- Notifications (même contrat que toast-manager.ts d'atomic : window.__starwind__.toast) ---------- */
  var toasts = {};
  function viewport() { return document.querySelector('[data-slot="toast-viewport"]'); }

  function fillToast(el, o) {
    var title = el.querySelector('[data-toast-title-text]');
    var desc = el.querySelector('[data-slot="toast-description"]');
    if (title) title.textContent = o.title || '';
    if (desc) desc.textContent = o.description || '';
    var old = el.querySelector('[data-toast-action]');
    if (old) old.remove();
    if (o.action) {
      var b = document.createElement('button');
      b.type = 'button';
      b.setAttribute('data-toast-action', '');
      b.setAttribute('data-slot', 'button');
      b.setAttribute('data-variant', 'outline');
      b.setAttribute('data-size', 'sm');
      b.textContent = o.action.label;
      b.addEventListener('click', function () { o.action.onClick(); });
      var content = el.querySelector('[data-slot="toast-content"]') || el;
      content.appendChild(b);
    }
  }

  function buildToast(vp, o) {
    var tpl = vp.querySelector('template[data-toast-template="' + (o.variant || 'default') + '"]') || vp.querySelector('template[data-toast-template="default"]');
    var el = tpl ? tpl.content.firstElementChild.cloneNode(true) : document.createElement('div');
    if (!tpl) { el.setAttribute('data-slot', 'toast'); el.setAttribute('role', 'status'); }
    fillToast(el, o);
    return el;
  }

  function schedule(t) {
    clearTimeout(t.timer);
    var vp = viewport();
    var d = t.options.duration !== undefined ? t.options.duration : parseInt((vp && vp.getAttribute('data-duration')) || '5000', 10);
    if (d > 0) t.timer = setTimeout(function () { manager.close(t.id); }, d);
  }

  var manager = {
    add: function (o) {
      var vp = viewport();
      if (!vp) return '';
      var tid = o.id || id('toast');
      var el = buildToast(vp, o);
      el.setAttribute('data-toast-id', tid);
      vp.appendChild(el);
      toasts[tid] = { id: tid, el: el, options: o };
      schedule(toasts[tid]);
      var limit = parseInt(vp.getAttribute('data-limit') || '3', 10);
      var live = Object.keys(toasts).filter(function (k) { return toasts[k].el.getAttribute('data-state') !== 'closed'; });
      while (live.length > limit) manager.close(live.shift());
      return tid;
    },
    update: function (tid, o) {
      var t = toasts[tid];
      if (!t) return;
      t.options = Object.assign({}, t.options, o);
      if (o.variant) {
        var fresh = buildToast(viewport(), t.options);
        fresh.setAttribute('data-toast-id', tid);
        t.el.replaceWith(fresh);
        t.el = fresh;
      } else fillToast(t.el, t.options);
      schedule(t);
    },
    close: function (tid) {
      var t = toasts[tid];
      if (!t) return;
      clearTimeout(t.timer);
      t.el.setAttribute('data-state', 'closed');
      if (t.options.onClose) t.options.onClose();
      setTimeout(function () {
        t.el.remove();
        delete toasts[tid];
        if (t.options.onRemove) t.options.onRemove();
      }, 200);
    },
    closeAll: function () { Object.keys(toasts).forEach(manager.close); },
  };
  window.__starwind__ = window.__starwind__ || {};
  window.__starwind__.toast = manager;

  document.addEventListener('click', function (e) {
    var c = closest(e.target, '[data-slot="toast-close"]');
    var el = c && closest(c, '[data-slot="toast"]');
    if (el && el.getAttribute('data-toast-id')) manager.close(el.getAttribute('data-toast-id'));
    else if (el) el.remove();
  });

  function init(root) {
    root = root || document;
    initTabs(root);
    initDialogs(root);
    initDialogsExt(root);
    initTooltips(root);
    initAccordions(root);
    initCollapsibles(root);
    initFloating(root);
    initSelects(root);
    initToggles(root);
    initRadioGroups(root);
    initOtps(root);
    initCarousels(root);
    initSidebars(root);
  }

  window.DesignKit = { init: init, carousel: carousel, toast: manager };
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', function () { init(document); });
  else init(document);
  document.addEventListener('astro:after-swap', function () { init(document); });
})();
