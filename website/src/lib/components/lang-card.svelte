<script lang="ts">
	import { MAX_LANG_LEVEL } from '$lib/constants';
	import { type LanguageEntry } from '$lib/models/content';

	type Props = {
		lang: LanguageEntry;
	};
	let { lang }: Props = $props();
</script>

<div class="card bg-base-300 rounded space">
	<div class="card-body text-neutral-content">
		<h4 class="card-title">{lang.name}</h4>
		<progress class="progress progress-primary" value={lang.level} max={MAX_LANG_LEVEL}></progress>
		{#if lang.comment}
			<p>{lang.comment}</p>
		{/if}
	</div>
</div>
