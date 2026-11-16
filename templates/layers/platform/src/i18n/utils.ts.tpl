@@meta when=messages@@
import { DEFAULT_LOCALE, toLocale, type Locale } from './config';
import defaultMessages from './messages/@@default_locale@@.json';
@@if i18n_fr@@
import fr from './messages/fr.json';
@@end@@
@@if i18n_en@@
import en from './messages/en.json';
@@end@@
@@if i18n_es@@
import es from './messages/es.json';
@@end@@
@@if i18n_ar@@
import ar from './messages/ar.json';
@@end@@

/** Forme des messages : celle de la langue par défaut (les autres langues doivent avoir les mêmes clés — testé). */
export type Messages = typeof defaultMessages;

const catalog = {
@@if i18n_fr@@
  fr,
@@end@@
@@if i18n_en@@
  en,
@@end@@
@@if i18n_es@@
  es,
@@end@@
@@if i18n_ar@@
  ar,
@@end@@
} as unknown as Record<Locale, Messages>;

export function getMessages(locale?: string): Messages {
  return catalog[toLocale(locale)] ?? catalog[DEFAULT_LOCALE];
}
