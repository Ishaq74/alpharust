<?php

namespace App\View;

/**
 * Calculs partagés par les composants du design kit (curseur, vidéo) : mêmes résultats que les atomes Astro.
 * Généré par tools/dsgen ; ne pas modifier à la main.
 */
final class DesignKit
{
    /** @return list<array{i:int, v:float|int, style:string, name:?string}> */
    public static function sliderThumbs(mixed $initial, float|int $min, float|int $max, string $orientation, ?string $name): array
    {
        $values = self::values($initial, $min, $max);
        $range = count($values) > 1;
        $thumbs = [];
        foreach ($values as $i => $v) {
            $pct = self::pct($v, $min, $max);
            $thumbs[] = [
                'i' => $i,
                'v' => $v,
                'style' => $orientation === 'horizontal' ? "left: {$pct}%" : "bottom: {$pct}%",
                'name' => $name === null ? null : ($range ? "{$name}[{$i}]" : $name),
            ];
        }

        return $thumbs;
    }

    public static function sliderRange(mixed $initial, float|int $min, float|int $max, string $orientation): string
    {
        $values = self::values($initial, $min, $max);
        $start = count($values) > 1 ? self::pct(min($values), $min, $max) : 0;
        $end = self::pct(max($values), $min, $max);
        $size = $end - $start;

        return $orientation === 'horizontal' ? "left: {$start}%; width: {$size}%" : "bottom: {$start}%; height: {$size}%";
    }

    public static function youtubeEmbed(string $src, bool $autoplay, bool $muted, bool $loop, bool $controls): ?string
    {
        if (! str_contains($src, 'youtube.com') && ! str_contains($src, 'youtu.be') && ! str_contains($src, 'youtube-nocookie.com')) {
            return null;
        }
        $patterns = ['#youtube\.com/shorts/([^?&]+)#', '#youtube\.com/watch\?v=([^&]+)#', '#youtube\.com/embed/([^?&]+)#', '#youtu\.be/([^?&]+)#', '#youtube-nocookie\.com/embed/([^?&]+)#'];
        foreach ($patterns as $pattern) {
            if (preg_match($pattern, $src, $m)) {
                $params = array_filter([
                    'autoplay' => $autoplay ? '1' : null,
                    'mute' => $muted ? '1' : null,
                    'loop' => $loop ? '1' : null,
                    'playlist' => $loop ? $m[1] : null,
                    'controls' => $controls ? null : '0',
                ]);
                $query = http_build_query($params);

                // alpharust-ignore: no-hardcoded-urls
                return 'https://www.youtube-nocookie.com/embed/'.$m[1].($query ? '?'.$query : '');
            }
        }

        return null;
    }

    /** @return list<float|int> */
    private static function values(mixed $initial, float|int $min, float|int $max): array
    {
        if (is_string($initial)) {
            $initial = array_map('floatval', array_filter(explode(',', $initial), 'strlen'));
        }

        return array_values(array_map(fn ($v) => min($max, max($min, $v + 0)), (array) $initial));
    }

    private static function pct(float|int $v, float|int $min, float|int $max): float|int
    {
        $span = ($max - $min) ?: 1;

        return round((($v - $min) / $span) * 100, 4);
    }
}
