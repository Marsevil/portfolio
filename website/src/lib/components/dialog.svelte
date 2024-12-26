<script lang="ts">
	import { type Snippet, getContext } from 'svelte';

	import type { Vars } from '$lib/models';

	type Props = {
		position: 'left' | 'right';
		children?: Snippet;
	};
	let { position, children }: Props = $props();

	let closed = $state(false);

	const { close: closeText } = getContext<Vars>('vars').dialogBox;
	const dialogSideClass: 'chat-start' | 'chat-end' = (() => {
		switch (position) {
			case 'left':
				return 'chat-start';
			case 'right':
				return 'chat-end';
		}
	})();

	function closeDialog() {
		closed = true;
	}
</script>

{#if !closed}
	<div class="chat {dialogSideClass}">
		<div class="chat-bubble chat-bubble-info">
			{@render children?.()}

			<button class="btn" onclick={closeDialog}>{closeText}</button>
		</div>
	</div>
{/if}
