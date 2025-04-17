<script lang="ts">
	import { type Snippet, getContext } from 'svelte';

	import type { Vars } from '$lib/models';

	type Props = {
		severity?: 'info' | 'warning';
		position: 'left' | 'right';
		children?: Snippet;
	};
	let { severity = 'info', position, children }: Props = $props();

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
	const chatBubbleClass: 'chat-bubble-warning' | 'chat-bubble-info' = (() => {
		switch (severity) {
			case 'info':
				return 'chat-bubble-info';
			case 'warning':
				return 'chat-bubble-warning';
		}
	})();

	function closeDialog() {
		closed = true;
	}
</script>

{#if !closed}
	<div class="chat {dialogSideClass}">
		<div class="chat-bubble {chatBubbleClass}">
			{@render children?.()}

			<button class="btn" onclick={closeDialog}>{closeText}</button>
		</div>
	</div>
{/if}
