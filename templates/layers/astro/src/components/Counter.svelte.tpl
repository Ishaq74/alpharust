@@meta when=svelte&!official@@
<script lang="ts">
  let count = $state(0);
</script>

<button type="button" class="button" onclick={() => count++}>Cliqué {count} fois</button>
