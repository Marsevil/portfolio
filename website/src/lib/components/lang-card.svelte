<script lang="ts">
	import { MAX_LANG_LEVEL } from '$lib/constants';
	import { type LanguageEntry } from '$lib/models/content';

	type Props = {
		lang: LanguageEntry;
	};
	let { lang }: Props = $props();
</script>

<div class="card bg-neutral rounded space">
	<div class="card-body">
		<h4 class="card-title">{lang.name}</h4>
		<progress class="progress progress-accent" value={lang.level} max={MAX_LANG_LEVEL}></progress>
		{#if lang.comment}
			<p class="text-accent">{lang.comment}</p>
		{/if}
	</div>
</div>
