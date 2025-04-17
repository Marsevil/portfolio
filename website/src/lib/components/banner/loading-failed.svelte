<script lang="ts">
	import type { Vars } from '$lib/models';
	import { getContext } from 'svelte';

	import Dialog from '$lib/components/dialog.svelte';

	const { loadingFailedText } = getContext<Vars>('vars').banner;
</script>

<div class="absolute bottom-0 right-0">
	<Dialog severity="warning" position="right">{loadingFailedText}</Dialog>
</div>
