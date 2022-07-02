import { writable } from 'svelte/store';
import pSBC from './colorUtil';

type wsocket = {
	socket: WebSocket | null;
	loading: boolean;
};

const socket = writable<wsocket>({
	socket: null,
	loading: true
});

const rooms = writable<string[]>([]);
const playerColor = writable('#ffff');
const playerColorLight = writable('#ffff');
const playerColorDark = writable('#ffff');

const boardArray: Array<string> = [];

// for (let i = 0; i < 3; i++) {
// 	for (let j = 0; j < 5; j++) {
// 		boardArray.push({ x: j, y: i });
// 	}
// }

const board = writable(boardArray);

const connect = () => {
	const lws =
		process.env.NODE_ENV === 'production'
			? window.location.protocol === 'http:'
				? new WebSocket(`ws://${window.location.host}/ws`)
				: new WebSocket(`wss://${window.location.host}/ws`)
			: new WebSocket(`ws://${import.meta.env.VITE_BACKEND}/ws`);

	console.log(`backend ${lws.url}`);
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
			const result: Array<string> = JSON.parse(event.data.split('\n')[1]);
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
		} else if (event.data && event.data.startsWith('color')) {
			console.log(event.data);
			console.log(event.data.split('\n'));
			const result: string = event.data.split('\n')[1];
			console.log('got color', result);
			playerColor.update(() => result);
			playerColorLight.update(() => pSBC(0.1, result) as string);
			playerColorDark.update(() => pSBC(-0.1, result) as string);
		} else {
			const result: { tile_num: number; color: string } = JSON.parse(event.data.split('\n')[1]);
			board.update((board) =>
				board.map((boardTile, index) => {
					if (index != result.tile_num) return boardTile;
					return result.color;
				})
			);
		}
	});
	return lws;
};

export { socket, connect, board, rooms, playerColor, playerColorLight, playerColorDark };
