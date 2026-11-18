@props(['src' => '', 'title' => 'Video', 'autoplay' => false, 'muted' => false, 'loop' => false, 'controls' => true, 'poster' => null])
@php
    $embed = \App\View\DesignKit::youtubeEmbed($src, $autoplay, $muted, $loop, $controls);
    $isShort = str_contains($src, '/shorts/');
@endphp
@if ($embed)
<iframe {{ $attributes->merge(['src' => $embed, 'title' => $title, 'allow' => 'accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share', 'referrerpolicy' => 'strict-origin-when-cross-origin', 'allowfullscreen' => '', 'data-video-type' => ($isShort ? 'youtube-shorts' : 'youtube'), 'data-slot' => 'video']) }}></iframe>
@else
<video {{ $attributes->merge(['src' => $src, 'autoplay' => ($autoplay ? '' : null), 'muted' => ($muted ? '' : null), 'loop' => ($loop ? '' : null), 'controls' => ($controls ? '' : null), 'poster' => $poster, 'data-slot' => 'video']) }}><track kind="captions"></video>
@endif
