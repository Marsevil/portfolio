<script lang="ts">
	import { getContext } from 'svelte';

	import type { Content, Vars } from '$lib/models';
	import type { LoadingState } from '$lib/models/enums';

	import Dialog from '$lib/components/dialog.svelte';

	import Title from './title.svelte';
	import Background from './background.svelte';
	import Loading from './loading.svelte';

	let backgroundLoadingState: LoadingState = $state('isLoading');

	const { name, title } = getContext<Content>('content').personal;
	const { interactDialog: interactDialogText, loadingFailed: loadingFailedText } =
		getContext<Vars>('vars').banner;
</script>

<div id="banner" class="w-full h-screen">
	<Background bind:loadingState={backgroundLoadingState} />
	{#if backgroundLoadingState !== 'isLoading'}
		<Title {name} {title} />
		<div class="absolute bottom-0 right-0">
			{#if backgroundLoadingState === 'loaded'}
				<Dialog severity="info" position="right">
					{interactDialogText}
				</Dialog>
			{:else}
				<Dialog severity="error" position="right">
					{loadingFailedText}
				</Dialog>
			{/if}
		</div>
	{/if}
	{#if backgroundLoadingState === 'isLoading'}
		<Loading />
	{/if}
</div>

<style>
</style>
