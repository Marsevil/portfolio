import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { ViteToml } from 'vite-plugin-toml';

export default defineConfig({
	plugins: [ViteToml(), sveltekit()],
	server: {
		fs: {
			allow: ['../interactive-background']
		}
	}
});
