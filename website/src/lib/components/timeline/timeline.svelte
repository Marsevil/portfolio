<script lang="ts">
	import { type EducationEntry, type ProfessionalEntry } from '$lib/models/content';

	import EducationItem from './education-item.svelte';
	import ProfessionalItem from './professional-item.svelte';

	interface Props {
		educations: EducationEntry[];
		experiences: ProfessionalEntry[];
	}
	let { educations, experiences }: Props = $props();

	const dynEntries: (
		| { type: 'edu'; obj: EducationEntry }
		| { type: 'pro'; obj: ProfessionalEntry }
	)[] = (() => {
		let value: typeof dynEntries = [];
		for (let edu of educations) {
			value.push({
				type: 'edu',
				obj: edu
			});
		}
		for (let exp of experiences) {
			value.push({
				type: 'pro',
				obj: exp
			});
		}
		value = value.sort((a, b) => {
			if (a.obj.period[0] < b.obj.period[0]) {
				return -1;
			} else if (a.obj.period[0] > b.obj.period[0]) {
				return 1;
			} else {
				return 0;
			}
		});
		return value;
	})();
</script>

<ul class="timeline timeline-snap-icon max-md:timeline-compact timeline-vertical">
	{#each dynEntries as ent, entIdx}
		{#if ent.type == 'edu'}
			<EducationItem
				entry={ent.obj}
				side={entIdx % 2 == 0 ? 'left' : 'right'}
				place={entIdx == 0 ? 'first' : entIdx == dynEntries.length - 1 ? 'last' : 'middle'}
			></EducationItem>
		{:else}
			<ProfessionalItem
				entry={ent.obj}
				side={entIdx % 2 == 0 ? 'left' : 'right'}
				place={entIdx == 0 ? 'first' : entIdx == dynEntries.length - 1 ? 'last' : 'middle'}
			></ProfessionalItem>
		{/if}
	{/each}
</ul>
