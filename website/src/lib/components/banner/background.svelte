<script lang="ts">
	import init, { run } from '@interactive-background/interactive_background';
	import { onMount } from 'svelte';

	import { type LoadingState } from '$lib/models/enums';

	const TIMEOUT_MS: number = 5000;
	const CANVAS_ID: string = 'interactive-background-renderer';

	type Props = {
		loadingState?: LoadingState;
	};
	let { loadingState = $bindable('isLoading') }: Props = $props();

	let canvas: HTMLCanvasElement;
	let canvasHidden = $derived(loadingState !== 'loaded');

	function _resizeCanvas() {
		canvas.width = window.innerWidth;
		canvas.height = window.innerHeight;
	}

	function onBackgroundLoaded() {
		loadingState = 'loaded';
	}

	onMount(() => {
		loadingState = 'isLoading';
		window.addEventListener('resize', _resizeCanvas, false);
		_resizeCanvas();

		Promise.race([
			init().then(() => run(CANVAS_ID, onBackgroundLoaded)),
			new Promise((_, reject) => setTimeout(() => reject(new Error('App timeout')), TIMEOUT_MS))
		]).catch((err: any) => {
			if (err instanceof Error && err.message.startsWith('Using exceptions for control flow')) {
				return;
			}

			loadingState = 'failed';
			console.error(err);
		});
	});
</script>

<canvas class:canvasHidden bind:this={canvas} id={CANVAS_ID} width={32} height={32}></canvas>

<style>
	canvas {
		width: 100%;
		height: 100%;
		background-color: black;
		transition: all ease-in-out 1s;
		opacity: 1;
	}

	canvas.canvasHidden {
		opacity: 0;
	}
</style>
