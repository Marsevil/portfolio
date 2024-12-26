<script lang="ts">
	import { setContext } from 'svelte';

	import { type Content, type Vars } from '$lib/models';

	import Banner from '$lib/components/banner/banner.svelte';
	import ContentComponent from '$lib/components/content/content.svelte';
	import Footer from '$lib/components/footer.svelte';

	import _data from '$lib/assets/content_fr.toml';
	import _vars from '$lib/assets/var_fr.toml';

	const data: Content = _data;
	const vars: Vars = _vars;

	setContext('vars', vars);
	setContext('content', data);
</script>

<Banner />
<ContentComponent />
<Footer />
