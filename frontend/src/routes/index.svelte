<script context="module" lang="ts">
	export const prerender = true;
</script>

<script lang="ts">
	import Grid from '$lib/Grid.svelte';
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<section>
	<Grid />
</section>

<style>
</style>
