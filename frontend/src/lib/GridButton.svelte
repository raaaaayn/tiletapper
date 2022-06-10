<script lang="ts">
	import { socket } from '../store';
	import { get } from 'svelte/store';
	export let tile_num: number;
	export let color: string;
	function sendMessage() {
		const store = get(socket);
		if (store.socket) store.socket.send(JSON.stringify({ tile_num }));
	}
</script>

<button
	class="h-20 shrink-1 grow-1 grid-button border border-gray-300 sm:border-6"
	on:click={sendMessage}
	style="background-color: {color || 'white'};"
>
	{tile_num}
</button>

<style>
	.grid-button {
		flex-basis: 33%;
	}
	@media (min-width: 40em) {
		.grid-button {
			flex-basis: 18%;
		}
	}
</style>
