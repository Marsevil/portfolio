<script lang="ts">
	import { getContext } from 'svelte';

	import type { Content, Vars } from '$lib/models';
	import type { LoadingState } from '$lib/models/enums';

	import Title from './title.svelte';
	import Background from './background.svelte';
	import Dialog from '$lib/components/dialog.svelte';

	let backgroundLoadingState: LoadingState = $state('isLoading');

	const { name, title } = getContext<Content>('content').personal;
	const { interactDialog: interactDialogText } = getContext<Vars>('vars').banner;
</script>

<div id="banner">
	<Background bind:loadingState={backgroundLoadingState} />
	{#if backgroundLoadingState === 'loaded'}
		<Title {name} {title} />
		<div class="absolute bottom-0 right-0">
			<Dialog position="right">{interactDialogText}</Dialog>
		</div>
	{:else}
		<div class="pointer-events-none absolute flex inset-0 -top-[400px] items-center justify-center">
			<div class="loading loading-spinner loading-lg"></div>
		</div>
	{/if}
</div>

<style>
	#banner {
		position: relative;
		top: 0px;
		left: 0px;
		width: 100vw;
		height: 100vh;
	}
</style>
