<script lang="ts">
	import { getContext } from 'svelte';

	import { type Content, type Vars } from '$lib/models';

	import LangCard from '$lib/components/lang-card.svelte';
	import ProjectCard from '$lib/components/project-card.svelte';
	import Timeline from '$lib/components/timeline/timeline.svelte';

	import Title from './title.svelte';

	const { titles: titleTexts } = getContext<Vars>('vars');
	const { humanLanguages, devLanguages, projects, educations, experiences, personal } =
		getContext<Content>('content');
</script>

<div class="container mx-auto" style="max-width: 1000px">
	<Title>{titleTexts.description}</Title>

	<p>{personal.description}</p>

	<Title>{titleTexts.language}</Title>

	<h3>{titleTexts.humanLanguage}</h3>
	<div class="inline-grid grid-cols-4 gap-4">
		{#each humanLanguages as lang}
			<LangCard {lang}></LangCard>
		{/each}
	</div>

	<h3>{titleTexts.devLanguage}</h3>
	<div class="inline-grid grid-cols-3 gap-4">
		{#each devLanguages as lang}
			<LangCard {lang}></LangCard>
		{/each}
	</div>

	<Title>{titleTexts.project}</Title>
	<div>
		{#each projects as project}
			<ProjectCard {project}></ProjectCard>
		{/each}
	</div>

	<Title>{titleTexts.road}</Title>
	<Timeline {educations} {experiences}></Timeline>
</div>

<style>
	h3 {
		margin: 1em 0em 1em 0em;
	}
</style>
