<script lang="ts">
	import { getContext } from 'svelte';

	import type { Content } from '$lib/models';
	import type { LoadingState } from '$lib/models/enums';

	import Title from './title.svelte';
	import Background from './background.svelte';
	import Loading from './loading.svelte';
	import Hud from './hud.svelte';
	import LoadingFailed from './loading-failed.svelte';

	let backgroundLoadingState: LoadingState = $state('isLoading');

	const { name, title } = getContext<Content>('content').personal;
</script>

<div id="banner" class="w-screen h-screen">
	<Background bind:loadingState={backgroundLoadingState} />
	{#if backgroundLoadingState !== 'isLoading'}
		<Title {name} {title} />
	{/if}
	{#if backgroundLoadingState === 'isLoading'}
		<Loading />
	{:else if backgroundLoadingState === 'loaded'}
		<Hud />
	{:else}
		<LoadingFailed />
	{/if}
</div>

<style>
</style>
