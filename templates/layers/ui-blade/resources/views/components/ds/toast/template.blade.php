@props(['variant' => 'default'])
<template {{ new \Illuminate\View\ComponentAttributeBag(['data-toast-template' => $variant]) }}>{{ $slot }}</template>
