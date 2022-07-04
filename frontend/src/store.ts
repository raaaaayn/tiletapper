import { writable } from 'svelte/store';
import pSBC from './colorUtil';

type wsocket = {
	socket: WebSocket | null;
	loading: boolean;
};

type GameMessage = {
	type: 'Color' | 'Tile' | 'Room' | 'Rooms' | 'Board';
	data: string;
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
		const data: GameMessage = JSON.parse(event.data);
		switch (data.type) {
			case 'Room': {
				rooms.update((rooms) => {
					return (rooms = [...rooms, data.data]);
				});
				break;
			}
			case 'Board': {
				const res: Array<string> = JSON.parse(data.data);
				board.update(() => res);
				break;
			}
			case 'Rooms': {
				const dataStripped = data.data.replace(']', '').replace('[', '').split(',');
				rooms.update(() => dataStripped);
				break;
			}
			case 'Color': {
				const result = data.data;
				console.log('got color', result);
				playerColor.update(() => result);
				playerColorLight.update(() => pSBC(0.1, result) as string);
				playerColorDark.update(() => pSBC(-0.1, result) as string);
				break;
			}
			case 'Tile': {
				const result: { tile_num: number; color: string } = JSON.parse(data.data);
				board.update((board) =>
					board.map((boardTile, index) => {
						if (index != result.tile_num) return boardTile;
						return result.color;
					})
				);
				break;
			}
			default:
				console.log('Invalid data');
				console.log(data);
		}
	});
	return lws;
};

export { socket, connect, board, rooms, playerColor, playerColorLight, playerColorDark };
