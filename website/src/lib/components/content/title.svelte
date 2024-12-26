<script lang="ts">
	interface Props {
		children?: import('svelte').Snippet;
	}

	let { children }: Props = $props();
</script>

<div id="root">
	<h2 class="text-accent">{@render children?.()}</h2>
	<div class="divider"></div>
</div>

<style>
	#root {
		margin: 2em 0em 2em 0em;
	}
</style>
