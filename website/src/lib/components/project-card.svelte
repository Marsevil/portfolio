<script lang="ts">
	import { getContext } from 'svelte';

	import type { ProjectEntry } from '$lib/models/content';
	import type { Vars } from '$lib/models/vars';

	interface Props {
		project: ProjectEntry;
	}
	let { project }: Props = $props();

	const { projectCard } = getContext<Vars>('vars');

	const [borderColorClass, badgeColorClass, badgeText]: [
		'border-success' | 'border-warning' | 'border-error',
		'badge-success' | 'badge-warning' | 'badge-error',
		string
	] = (() => {
		switch (project.state) {
			case 'done':
				return ['border-success', 'badge-success', projectCard.stateTag.done];
			case 'wip':
				return ['border-warning', 'badge-warning', projectCard.stateTag.wip];
			case 'aborted':
				return ['border-error', 'badge-error', projectCard.stateTag.aborted];
		}
	})();
</script>

<div class="card bg-neutral card-bordered {borderColorClass} rounded space my-4">
	<div class="card-body">
		<h4 class="card-title hover:text-primary">
			{#if project.link}
				<a href={project.link ?? ''} target="_blank"> {project.title} </a>
			{:else}
				{project.title}
			{/if}
			<div class="badge {badgeColorClass}">
				{badgeText}
			</div>
		</h4>
		<p>{project.description}</p>
		{#if project.sourceLink}
			<a class="italic hover:text-primary" href={project.sourceLink} target="_blank">
				{projectCard.sourceLink}
			</a>
		{/if}
		<div class="card-actions">
			{#each project.technos as tech}
				<span class="badge badge-info">{tech}</span>
			{/each}
		</div>
	</div>
</div>
