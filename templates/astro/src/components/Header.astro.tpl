---
const links = [
  { href: "/", label: "Accueil" },
  { href: "/blog", label: "Blog" },
  { href: "/about", label: "À propos" },
];
---
<header>
  <nav aria-label="Navigation principale">
    {links.map((l) => <a href={l.href}>{l.label}</a>)}
  </nav>
</header>
