<script lang="ts">
	import GridButton from './GridButton.svelte';
	import { get } from 'svelte/store';
	import { onMount } from 'svelte';
	import { connect, board, socket, rooms } from '../store';
	import { toast } from '@zerodevx/svelte-toast';

	let localboard = get(board);
	let localrooms = get(rooms);
	let loading = true;
	let error = false;
	let roomId: null | string = null;

	onMount(() => {
		try {
			connect();
			board.subscribe((changed) => {
				console.log({ changed });
				localboard = get(board);
				loading = false;
			});
			rooms.subscribe(() => {
				localrooms = get(rooms);
			});
		} catch (err) {
			console.log('Error happened');
			console.error(err);
			loading = false;
			error = true;
		}
	});

	function createRoom() {
		const store = get(socket);
		if (store.socket) {
			store.socket.send(JSON.stringify({ type: 'Create' }));
			toast.push('Created room');
		}
	}
	function joinRoom(roomid: string) {
		const store = get(socket);
		if (store.socket) {
			store.socket.send(JSON.stringify({ type: 'Join', data: parseInt(roomid) }));
			roomId = roomid;
		}
	}
	function exit() {
		const store = get(socket);
		if (store.socket) {
			store.socket.send(JSON.stringify({ type: `Exit` }));
			board.update(() => []);
		}
	}
</script>

<div>
	{#if loading}
		<div class="flex h-screen justify-center items-center">
			<svg width="89" height="89" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"
				><path
					d="M12,23a9.63,9.63,0,0,1-8-9.5,9.51,9.51,0,0,1,6.79-9.1A1.66,1.66,0,0,0,12,2.81h0a1.67,1.67,0,0,0-1.94-1.64A11,11,0,0,0,12,23Z"
					><animateTransform
						attributeName="transform"
						type="rotate"
						calcMode="linear"
						dur="0.75s"
						values="0 12 12;360 12 12"
						repeatCount="indefinite"
					/></path
				></svg
			>
		</div>
	{:else if error}
		<section class="flex h-screen justify-center items-center">
			<h1 class="p-[3rem] font-mono text-3xl font-semibold">Could not connect to server</h1>
		</section>
	{:else if localboard && localboard.length > 0}
		<div
			class="grid-buttons flex pr-7 pl-7 pt-5 justify-between items-center sm:mr-[50px] sm:ml-[50px]"
		>
			{#if roomId}
				<p
					class="px-6 p-3 max-w-sm bg-white text-black-600 font-semibold rounded-xl shadow-lg flex items-center border border-gray-200 space-x-4"
				>
					Room: {roomId}
				</p>
			{:else}
				<p
					class="px-6 p-3 max-w-sm bg-white text-black-600 font-semibold rounded-xl shadow-lg flex items-center border border-gray-200 space-x-4"
				>
					Room: 1231232
				</p>
			{/if}
			<button
				on:click={exit}
				class="px-6 p-3 max-w-sm bg-white text-black-600 font-semibold rounded-xl shadow-lg flex items-center border border-gray-200 space-x-4 hover:text-white hover:bg-purple-600 hover:border-transparent focus:outline-none focus:ring-2 focus:ring-purple-600 focus:ring-offset-2"
				>Home</button
			>
		</div>
		<div class="flex flex-wrap m-7 justify-center">
			{#each localboard as tile, i}
				<GridButton tile_num={i} color={tile} />
			{/each}
		</div>
	{:else}
		<div class="flex m-7 flex-col gap-9">
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
