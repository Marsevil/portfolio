<script lang="ts">
	import { getContext } from 'svelte';

	import type { Vars, Content } from '$lib/models';

	const { rightReserved }: Vars = getContext('vars');
	const {
		personal: { name }
	}: Content = getContext('content');
</script>

<footer class="footer p-4 bg-neutral text-neutral-content items-center">
	<p>Copyright © {new Date().getFullYear()} - {name} - {rightReserved}</p>
	<nav class="grid-flow-col gap-4 items-center justify-self-end">
		<a href="https://github.com/Marsevil" target="_blank">
			<img
				class="fill-current"
				height="40"
				width="40"
				src="github-white-icon.svg"
				alt="github icon"
			/>
		</a>
	</nav>
</footer>
