/**
 * Carrousel CSS pur : défilement natif (scroll-snap) piloté par ds.js. Même API que la version Starwind (Embla) :
 * `initCarousel(élément)` renvoie un petit contrôleur (précédent, suivant, aller à).
 */
export interface CarouselOptions {
  loop?: boolean;
  align?: "start" | "center" | "end";
  startIndex?: number;
}

export interface CarouselApi {
  scrollPrev(): void;
  scrollNext(): void;
  scrollTo(index: number): void;
  selectedScrollSnap(): number;
}

/** Nom de l'API dans la version Starwind (Embla). */
export type CarouselManager = CarouselApi;

type DesignKit = { carousel?: (el: HTMLElement) => CarouselApi };

export function initCarousel(el: HTMLElement): CarouselApi | undefined {
  const kit = (window as unknown as { DesignKit?: DesignKit }).DesignKit;
  return kit?.carousel?.(el);
}
