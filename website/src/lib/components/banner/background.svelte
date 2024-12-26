<script lang="ts">
	import init from '@interactive-background/interactive-background';
	import { onMount } from 'svelte';

	import { type LoadingState } from '$lib/models/enums';

	type Props = {
		loadingState?: LoadingState;
	};
	let { loadingState = $bindable('isLoading') }: Props = $props();

	let canvas: HTMLCanvasElement;

	function _resizeCanvas() {
		canvas.width = window.innerWidth;
		canvas.height = window.innerHeight;
	}

	onMount(() => {
		loadingState = 'isLoading';
		window.addEventListener('resize', _resizeCanvas, false);
		_resizeCanvas();

		init()
			.catch((err) => {
				loadingState = 'failed';
				console.error(err);
			})
			.then(() => {
				loadingState = 'loaded';
				console.log('background interactive started...');
			});
	});
</script>

<canvas bind:this={canvas} id="interactive-background-renderer" width={32} height={32}></canvas>

<style>
	canvas {
		width: 100%;
		height: 100%;
		background-color: black;
	}
</style>
