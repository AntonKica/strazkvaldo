import adapter from '@sveltejs/adapter-node';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),
	kit: {
		adapter: adapter({
			fallback: 'index.html' // may differ from host to host

		}),
		prerender: {
			entries: ['*']
		},
		paths: {
			base: '/ui'
		}
	}
};

export default config;
