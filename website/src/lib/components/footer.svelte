<script lang="ts">
	import { getContext } from 'svelte';

	import type { Vars, Content } from '$lib/models';

	const { rightReserved } = getContext<Vars>('vars');
	const {
		personal: { name, links }
	} = getContext<Content>('content');
</script>

<footer class="footer p-4 bg-base-300 items-center">
	<p>Copyright © {new Date().getFullYear()} - {name} - {rightReserved}</p>
	<nav class="grid-flow-col gap-4 items-center justify-self-end">
		{#if links.linkedin}
			<a href={links.linkedin} target="_blank">
				<img
					class="fill-current"
					height="40"
					width="40"
					src="linkedin-icon.svg"
					alt="linkedin icon"
				/>
			</a>
		{/if}
		{#if links.escapegamefr}
			<a href={links.escapegamefr} target="_blank">
				<img
					class="fill-current"
					height="40"
					width="40"
					src="escapegamefr-icon.svg"
					alt="escapegame.fr icon"
				/>
			</a>
		{/if}
		{#if links.github}
			<a href={links.github} target="_blank">
				<img
					class="fill-current"
					height="40"
					width="40"
					src="github-white-icon.svg"
					alt="github icon"
				/>
			</a>
		{/if}
		{#if links.gitlab}
			<a href={links.gitlab} target="_blank">
				<img class="fill-current" height="40" width="40" src="gitlab-icon.svg" alt="gitlab icon" />
			</a>
		{/if}
	</nav>
</footer>
