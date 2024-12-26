<script lang="ts">
	interface Props {
		side: 'left' | 'right';
		place: 'first' | 'last' | 'middle';
		color: 'primary' | 'secondary';
		time?: import('svelte').Snippet;
		title?: import('svelte').Snippet;
		location?: import('svelte').Snippet;
		content?: import('svelte').Snippet;
	}

	let { side, place, color, time, title, location, content }: Props = $props();

	const [textColorClass, borderColorClass]: [
		'text-primary' | 'text-secondary',
		'border-primary' | 'border-secondary'
	] = (() => {
		switch (color) {
			case 'primary':
				return ['text-primary', 'border-primary'];
			case 'secondary':
				return ['text-secondary', 'border-secondary'];
		}
	})();
</script>

<li>
	{#if place !== 'first'}
		<hr />
	{/if}
	<div class="timeline-middle {textColorClass}">
		<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" class="h-5 w-5">
			<circle cx="10" cy="10" r="8" />
		</svg>
	</div>

	<div
		class="{side == 'left' ? 'timeline-start' : 'timeline-end'} mb-10 {side == 'left'
			? 'md:text-end'
			: ''}"
	>
		<time class="font-bold">{@render time?.()}</time>

		<div
			class="card card-bordered {borderColorClass} border-2 bg-neutral space rounded text-left m-3"
		>
			<div class="card-body">
				<div class="card-title">{@render title?.()}</div>
				<div class="italic">{@render location?.()}</div>

				{@render content?.()}
			</div>
		</div>
	</div>
	{#if place !== 'last'}
		<hr />
	{/if}
</li>
