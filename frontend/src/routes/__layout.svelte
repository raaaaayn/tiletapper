<script lang="ts">
	import '../app.css';
	import { SvelteToast } from '@zerodevx/svelte-toast';
	const options = {
		duration: 1000, // duration of progress bar tween to the `next` value
		dismissable: true, // allow dismiss with close button
		intro: { y: 192 } // toast intro fly animation settings
	};
</script>

<main class="bg:dark h-screen overflow-auto bg-gradient-to-br from-gray-50 to-gray-300">
	<slot />
	<SvelteToast {options} />
</main>

<footer />

<style>
	:root {
		--toastContainerTop: auto;
		--toastContainerRight: auto;
		--toastContainerBottom: 2rem;
		--toastContainerLeft: calc(50vw - 8rem);
	}
</style>
