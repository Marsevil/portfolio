<script lang="ts">
	import { type EducationEntry } from '$lib/models/content';

	import TimelineItem, { type Props as BaseProps } from './timeline-item.svelte';

	export type Props = {
		entry: EducationEntry;
		side: BaseProps['side'];
		place: BaseProps['place'];
	};

	let { entry, side, place }: Props = $props();
</script>

<TimelineItem {side} {place} borderColorClass="border-education" pointColorClass="text-education">
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
