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
	const lws = new WebSocket('ws://localhost:9001/ws');
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
			const result: Array<{ tile_num: number; color: string }> = JSON.parse(
				event.data.split('\n')[1]
			);
			console.log(boardArray);
			board.update(() => result);
		} else if (event.data && event.data.startsWith('rooms')) {
			rooms.update(() => event.data.split('\n').slice(1, -1));
		} else {
			const result: { tile_num: number; color: string } = JSON.parse(event.data.split('\n')[1]);
			console.log({ result });
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
