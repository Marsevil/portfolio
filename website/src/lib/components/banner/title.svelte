<script lang="ts">
	import { onMount } from 'svelte';

	interface Props {
		title: string;
		name: string;
	}

	let { title, name }: Props = $props();

	const SPLIT_LETTERS_CLASSNAME: string = 'js-split-letters';
	const HIDE_ON_HOVER_CLASSNAME: string = 'hide-on-hover';

	function _splitLetters() {
		let elems = document.getElementsByClassName(SPLIT_LETTERS_CLASSNAME);
		for (const elem of elems) {
			const text = elem.textContent;
			if (text === null) {
				continue;
			}

			const newContent = text.split('').map((letter) => {
				const span = document.createElement('span');
				span.textContent = letter;
				span.classList.add(HIDE_ON_HOVER_CLASSNAME);
				return span;
			});

			elem.innerHTML = newContent.reduce((str, elem) => str + elem.outerHTML, '');
		}
	}

	onMount(() => {
		_splitLetters();
	});
</script>

<div id="banner-title">
	<h1 class={SPLIT_LETTERS_CLASSNAME}>{name}</h1>
	<h2 class={SPLIT_LETTERS_CLASSNAME}>{title}</h2>
</div>

<style>
	h1 {
		font-size: 4em;
	}
	h2 {
		font-size: 2em;
	}

	#banner-title {
		position: absolute;

		top: 0;
		left: 0;

		width: 100%;
		height: 100%;

		display: grid;
		align-content: center;
		text-align: center;

		pointer-events: none;
		z-index: 0;

		color: white;

		animation: 2s ease 0s 1 normal text_appears;
	}

	:global(.hide-on-hover :hover) {
		opacity: 0;
	}

	@keyframes text_appears {
		from {
			opacity: 0;
			transform: translateY(-10%);
		}
		to {
			opacity: 100%;
			transform: translateY(0);
		}
	}
</style>
