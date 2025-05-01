<script lang="ts">
	import { getContext } from 'svelte';

	import type { ProjectEntry } from '$lib/models/content';
	import type { Vars } from '$lib/models/vars';

	import GitIcon from '$lib/icons/git-icon.svelte';

	interface Props {
		project: ProjectEntry;
	}
	let { project }: Props = $props();

	const { projectCard: projectCardTexts } = getContext<Vars>('vars');

	const badgeText: string = (() => {
		switch (project.state) {
			case 'done':
				return projectCardTexts.stateTag.done;
			case 'wip':
				return projectCardTexts.stateTag.wip;
			case 'aborted':
				return projectCardTexts.stateTag.aborted;
		}
	})();
</script>

<div class="card bg-base-300 rounded space my-4">
	<div class="card-body">
		<h4 class="card-title">
			{#if project.link}
				<a class="hover:text-accent" href={project.link} target="_blank"> {project.title} </a>
			{:else}
				{project.title}
			{/if}
			{#if project.sourceLink}
				<a href={project.sourceLink} target="_blank" aria-labelledby="git icon">
					<GitIcon class="fill-current hover:fill-accent" height="1em" />
				</a>
			{/if}
			<div class="badge bg-base-content text-base-100">
				{badgeText}
			</div>
		</h4>
		<p>{project.description}</p>
		<div class="card-actions">
			{#each project.technos as tech}
				<span class="badge badge-primary">{tech}</span>
			{/each}
		</div>
	</div>
</div>
