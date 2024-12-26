<script lang="ts">
	import { type EducationEntry } from '$lib/models/content';

	import TimelineItem from './timeline-item.svelte';

	interface Props {
		entry: EducationEntry;
		side: 'left' | 'right';
		place: 'first' | 'last' | 'middle';
	}

	let { entry, side, place }: Props = $props();
</script>

<TimelineItem {side} {place} color="primary">
	{#snippet time()}
		{entry.period[0].toLocaleString('fr', { year: 'numeric' })} &rarr; {entry.period[1].toLocaleString(
				'fr',
				{ year: 'numeric' }
			)}
	{/snippet}
	{#snippet title()}
		{entry.title}
	{/snippet}
	{#snippet location()}
		{entry.institution} - {entry.location}
	{/snippet}
	{#snippet content()}
	
			{entry.comment ?? ''}
		
	{/snippet}
</TimelineItem>
