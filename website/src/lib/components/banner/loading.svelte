<script lang="ts">
	import { type Vars } from '$lib/models';
	import { getContext } from 'svelte';

	const { loadingText } = getContext<Vars>('vars').banner;
</script>

<div class="pointer-events-none absolute flex inset-0 items-center justify-center">
	<div class="text-4xl flex">
		<span>{loadingText}</span>
		<span class="bounce-pause">.</span>
		<span class="bounce-pause-100">.</span>
		<span class="bounce-pause-200">.</span>
		<!--<span class="loading loading-dots loading flex bottom-200"></span>-->
	</div>
</div>

<style>
	.bounce-pause {
		animation: bounce-pause 1s ease-in-out infinite 0ms;
	}
	.bounce-pause-100 {
		animation: bounce-pause 1s ease-in-out infinite 100ms;
	}
	.bounce-pause-200 {
		animation: bounce-pause 1s ease-in-out infinite 200ms;
	}
	@keyframes bounce-pause {
		0% {
			transform: translateY(0);
		}
		25% {
			transform: translateY(-0.15em);
		}
		50% {
			transform: translateY(0);
		}
	}
</style>
