import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// for more information about preprocessors
	preprocess: preprocess({
		postcss: true,
		replace: [[/process\.env\.NODE_ENV/g, JSON.stringify(process.env.NODE_ENV)]]
	}),

	kit: {
		adapter: adapter()
	}
};

export default config;
