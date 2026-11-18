@props(['isUploading' => false, 'id' => null])
<label {{ new \Illuminate\View\ComponentAttributeBag(['id' => $id, 'tabindex' => '0', 'data-slot' => 'dropzone', 'data-drag-active' => 'false', 'data-has-files' => 'false', 'data-is-uploading' => ($isUploading ? 'true' : 'false')]) }}>
@if (trim((string) $slot) === '')
<div data-slot="dropzone-upload-indicator"><svg xmlns="http://www.w3.org/2000/svg" width="2.5rem" height="2.5rem" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="cloud-upload"><path d="M7 18a4.6 4.4 0 0 1 0-9 5 4.5 0 0 1 11 2h1a3.5 3.5 0 0 1 0 7h-1"/><path d="m9 15 3-3 3 3M12 12v9"/></svg><p>Click to upload or drag and drop</p></div><div hidden data-slot="dropzone-loading-indicator"><svg xmlns="http://www.w3.org/2000/svg" width="2.5rem" height="2.5rem" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" data-icon="loader"><path d="M12 3a9 9 0 1 0 9 9"/></svg><p>Uploading file(s)...</p></div><div aria-live="polite" aria-label="Uploaded files" data-slot="dropzone-files-list"></div>
@else
{{ $slot }}
@endif
<input {{ $attributes->merge(['tabindex' => '-1', 'type' => 'file', 'class' => 'sr-only']) }}></label>
