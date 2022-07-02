<script lang="ts">
	import { socket } from '../store';
	import { get } from 'svelte/store';
	export let tile_num: number;
	export let tileColor: string;
	function sendMessage() {
		const store = get(socket);
		if (store.socket) store.socket.send(JSON.stringify({ type: 'Tile', data: tile_num }));
	}
</script>

<button
	class={`h-20 shrink-1 grow-1 grid-button border border-gray-300 sm:border-6 hover:bg-[${tileColor}]-600 bg-[var(--bg)]-1000`}
	on:click={sendMessage}
	style="background-color: {tileColor || 'white'};"
>
	{tile_num}
</button>

<style>
	.grid-button {
		flex-basis: 33%;
	}
	.grid-button:hover {
		background-color: var(--bg-light) !important;
	}
	.grid-button:active {
		background-color: var(--bg-dark) !important;
		/* background-color: #3e8e41; */
		/* box-shadow: 0 5px #666; */
		/* transform: translateY(4px); */
	}
	@media (min-width: 40em) {
		.grid-button {
			flex-basis: 18%;
		}
	}
</style>
