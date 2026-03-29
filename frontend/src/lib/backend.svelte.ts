import { browser } from '$app/environment';

export enum ConnectionStatus {
	Disconnected = 'Disconnected',
	Connecting = 'Connecting',
	Connected = 'Connected'
}

export const backend = $state({
	status: ConnectionStatus.Disconnected,
	message: ''
});

let socket: WebSocket | null = null;

export const connect = () => {
	if (!browser || socket) return;

	backend.status = ConnectionStatus.Connecting;
	socket = new WebSocket('ws://127.0.0.1:3000/ws');
	socket.onopen = () => {
		backend.status = ConnectionStatus.Connected;
	};

	socket.onmessage = (event) => {
		backend.message = event.data;
	};

	socket.onclose = () => {
		backend.status = ConnectionStatus.Disconnected;
	};
};

export const send = (msg: string) => {
	if (socket?.readyState === WebSocket.OPEN) {
		socket.send(msg);
	}
};
