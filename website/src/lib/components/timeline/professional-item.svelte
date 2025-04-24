<script lang="ts">
	import { type ProfessionalEntry } from '$lib/models/content';

	import TimelineItem, { type Props as BaseProps } from './timeline-item.svelte';

	type Props = {
		entry: ProfessionalEntry;
		side: BaseProps['side'];
		place: BaseProps['place'];
	};

	let { entry, side, place }: Props = $props();
</script>

<TimelineItem
	{side}
	{place}
	borderColorClass="border-professional"
	pointColorClass="text-professional"
>
	{#snippet time()}
		{entry.period[0].toLocaleString('fr', { month: 'short', year: 'numeric' })} &rarr; {entry.period[1].toLocaleString(
			'fr',
			{ month: 'short', year: 'numeric' }
		)}
	{/snippet}
	{#snippet title()}
		{entry.title}
	{/snippet}
	{#snippet location()}
		{entry.company} - {entry.location}
	{/snippet}
	{#snippet content()}
		{entry.description}

		<ul class="list-disc">
			{#each entry.tasks as task}
				<li>{task}</li>
			{/each}
		</ul>

		<div class="card-actions">
			{#each entry.technos as tech}
				<span class="badge badge-primary"> {tech} </span>
			{/each}
		</div>
	{/snippet}
</TimelineItem>
