<script lang="ts">
	import { type ProfessionalEntry } from '$lib/models/content';

	import TimelineItem from './timeline-item.svelte';

	interface Props {
		entry: ProfessionalEntry;
		side: 'left' | 'right';
		place: 'first' | 'last' | 'middle';
	}

	let { entry, side, place }: Props = $props();
</script>

<TimelineItem {side} {place} color="secondary">
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
					<span class="badge badge-info"> {tech} </span>
				{/each}
			</div>
		
	{/snippet}
</TimelineItem>
