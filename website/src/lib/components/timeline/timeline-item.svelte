<script lang="ts">
	import { type Snippet } from 'svelte';

	export type Props = {
		side: 'left' | 'right';
		place: 'first' | 'last' | 'middle';

		borderColorClass: string;
		pointColorClass: string;

		time?: Snippet;
		title?: Snippet;
		location?: Snippet;
		content?: Snippet;
	};

	let { side, place, borderColorClass, pointColorClass, ...snippets }: Props = $props();
</script>

<li>
	{#if place !== 'first'}
		<hr />
	{/if}
	<div class="timeline-middle {pointColorClass}">
		<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="h-5 w-5">
			<circle cx="10" cy="10" r="8" />
		</svg>
	</div>

	<div
		class="{side == 'left' ? 'timeline-start' : 'timeline-end'} mb-10 {side == 'left'
			? 'md:text-end'
			: ''}"
	>
		<time class="font-bold">{@render snippets.time?.()}</time>

		<div
			class="card card-bordered {borderColorClass} border-2 bg-neutral space rounded text-left m-3"
		>
			<div class="card-body text-neutral-content">
				<div class="card-title">{@render snippets.title?.()}</div>
				<div class="italic">{@render snippets.location?.()}</div>

				{@render snippets.content?.()}
			</div>
		</div>
	</div>
	{#if place !== 'last'}
		<hr />
	{/if}
</li>
