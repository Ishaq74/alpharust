@@meta when=messages&tests@@
import { describe, expect, it } from 'vitest';
import { LOCALES, DEFAULT_LOCALE, dirOf, localizedPath } from '@i18n/config';
import { getMessages } from '@i18n/utils';

function keys(value: unknown, prefix = ''): string[] {
  if (value && typeof value === 'object') {
    return Object.entries(value).flatMap(([k, v]) => keys(v, prefix ? `${prefix}.${k}` : k));
  }
  return [prefix];
}

describe('i18n', () => {
  const reference = keys(getMessages(DEFAULT_LOCALE)).sort();

  it.each(LOCALES)('la langue %s a exactement les mêmes clés que la langue par défaut', (locale) => {
    expect(keys(getMessages(locale)).sort()).toEqual(reference);
  });

  it.each(LOCALES)('la langue %s n’a aucune valeur vide', (locale) => {
    const empty = keys(getMessages(locale)).filter((k) => {
      let v: unknown = getMessages(locale);
      for (const part of k.split('.')) v = (v as Record<string, unknown>)[part];
      return typeof v !== 'string' || v.trim() === '';
    });
    expect(empty).toEqual([]);
  });

  it('préfixe uniquement les langues non par défaut', () => {
    expect(localizedPath(DEFAULT_LOCALE, '/a')).toBe('/a');
    for (const l of LOCALES.filter((x) => x !== DEFAULT_LOCALE)) {
      expect(localizedPath(l, '/a')).toBe(`/${l}/a`);
      expect(localizedPath(l, '/')).toBe(`/${l}/`);
    }
  });

  it('applique la direction du texte', () => {
    expect(dirOf('ar')).toBe('rtl');
    expect(dirOf('fr')).toBe('ltr');
  });
});
