<script lang="ts">
	import GridButton from './GridButton.svelte';
	import { get } from 'svelte/store';
	import { onMount } from 'svelte';
	import { connect, board, socket, rooms } from '../store';

	let localboard = get(board);
	let localrooms = get(rooms);
	let loading = true;
	onMount(() => {
		connect();
		board.subscribe((changed) => {
			console.log({ changed });
			localboard = get(board);
			loading = false;
		});
		rooms.subscribe(() => {
			localrooms = get(rooms);
		});
	});

	function createRoom() {
		const store = get(socket);
		if (store.socket) store.socket.send('create');
	}
	function joinRoom(roomid: string) {
		const store = get(socket);
		if (store.socket) store.socket.send(`join\n${roomid}`);
	}
	function exit() {
		const store = get(socket);
		if (store.socket) {
			store.socket.send(`exit`);
			board.update(() => []);
		}
	}
</script>

<div class="">
	{#if localboard && localboard.length > 0}
		<button
			on:click={exit}
			class="mt-3 px-6 p-3 max-w-sm mx-auto bg-white text-black-600 font-semibold rounded-xl shadow-lg flex items-center border border-gray-200 space-x-4 hover:text-white hover:bg-purple-600 hover:border-transparent focus:outline-none focus:ring-2 focus:ring-purple-600 focus:ring-offset-2"
			>Exit</button
		>
		<div class="flex flex-wrap m-7 justify-center">
			{#each localboard as tile, _i}
				<GridButton tile_num={tile.tile_num} color={tile.color} />
			{/each}
		</div>
	{:else if !loading}
		<div class="flex m-7 flex-col gap-9 ">
			<button
				on:click={createRoom}
				class="px-8 py-1 max-w-[30%] text-m text-black-600 font-semibold bg-gray-50 rounded border border-gray-200 hover:text-white hover:bg-purple-600 hover:border-transparent focus:outline-none focus:ring-2 focus:ring-purple-600 focus:ring-offset-2"
				>Create Room</button
			>
			<ul class="flex flex-col gap-5">
				{#if localrooms.length > 0}
					{#each localrooms as room}
						<li>
							<button
								class="px-8 py-1 w-[70%] text-m text-black-600 font-semibold bg-gray-50 rounded border border-gray-200 hover:text-white hover:bg-purple-600 hover:border-transparent focus:outline-none focus:ring-2 focus:ring-purple-600 focus:ring-offset-2"
								on:click={() => {
									joinRoom(room);
								}}>Join Room {room}</button
							>
						</li>
					{/each}
				{/if}
			</ul>
		</div>
	{/if}
</div>
