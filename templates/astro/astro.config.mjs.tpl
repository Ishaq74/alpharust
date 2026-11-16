import { defineConfig } from "astro/config";

// Configuration Astro. Renseigner `site` via SITE_URL pour les sitemaps / URLs canoniques.
export default defineConfig({
  site: process.env.SITE_URL,
});
