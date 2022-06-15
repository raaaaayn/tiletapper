import { writable } from 'svelte/store';

type wsocket = {
	socket: WebSocket | null;
	loading: boolean;
};

const socket = writable<wsocket>({
	socket: null,
	loading: true
});

const rooms = writable<string[]>([]);

const boardArray: Array<{ tile_num: number; color: string }> = [];

// for (let i = 0; i < 3; i++) {
// 	for (let j = 0; j < 5; j++) {
// 		boardArray.push({ x: j, y: i });
// 	}
// }

const board = writable(boardArray);

const connect = () => {
	console.log();
	const lws =
		process.env.NODE_ENV === 'production'
			? new WebSocket(`ws://${window.location.host}/ws`)
			: new WebSocket(`ws://${import.meta.env.VITE_BACKEND}/ws`);
	socket.update(() => ({
		socket: lws,
		loading: false
	}));

	// Connection opened
	lws.addEventListener('open', function () {
		console.log("It's open");
	});

	lws.addEventListener('message', function (event) {
		if (event.data && event.data.startsWith('board')) {
			console.log('got board', event.data);
			const result: Array<{ tile_num: number; color: string }> = JSON.parse(
				event.data.split('\n')[1]
			);
			board.update(() => result);
		} else if (event.data && event.data.startsWith('rooms')) {
			const data = event.data.split('\n');
			console.log('rooms updated');
			console.log(event.data);
			console.log(data);
			console.log(data.slice(1, data.length));
			rooms.update(() => data.slice(1, data.length));
		} else if (event.data && event.data.startsWith('Room')) {
			console.log(event.data);
			console.log(event.data.split('\n'));
			const result: string = JSON.parse(event.data.split('\n')[1]);
			console.log('got room', result);
			rooms.update((rooms) => {
				return (rooms = [...rooms, result]);
			});
		} else {
			const result: { tile_num: number; color: string } = JSON.parse(event.data.split('\n')[1]);
			console.log('got tile', result);
			board.update((board) =>
				board.map((tile) => {
					console.log(tile, result);
					if (tile.tile_num != result.tile_num) return tile;
					return result;
				})
			);
		}
	});
	return lws;
};

export { socket, connect, board, rooms };
