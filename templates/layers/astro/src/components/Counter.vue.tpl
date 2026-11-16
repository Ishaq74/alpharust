@@meta when=vue&!official@@
<script setup lang="ts">
import { ref } from 'vue';

const count = ref(0);
</script>

<template>
  <button type="button" class="button" @click="count++">Cliqué {{ count }} fois</button>
</template>
