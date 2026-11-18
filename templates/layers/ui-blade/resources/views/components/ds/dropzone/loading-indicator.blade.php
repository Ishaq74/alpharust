<div hidden data-slot="dropzone-loading-indicator">
@if (trim((string) $slot) === '')
<svg xmlns="http://www.w3.org/2000/svg" width="2.5rem" height="2.5rem" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="loader"><path d="M12 3a9 9 0 1 0 9 9"/></svg><p>Uploading file(s)...</p>
@else
{{ $slot }}
@endif
</div>
