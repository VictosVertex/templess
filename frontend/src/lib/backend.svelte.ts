import { browser } from '$app/environment';
import { WEBSOCKET_URL } from './constants';
import {
	OptimizationStatus,
	ServerMessageType,
	type ClientMessage,
	type OptimizationResult,
	type ServerMessage
} from './types';

export enum ConnectionStatus {
	Disconnected = 'Disconnected',
	Connecting = 'Connecting',
	Connected = 'Connected'
}

export class Backend {
	private socket: WebSocket | null = null;
	private socket_status: ConnectionStatus = $state(ConnectionStatus.Disconnected);
	optimizationStatus = $state(OptimizationStatus.Ready);
	lastMessage = $state<ServerMessage | null>(null);

	private onNewModelReceived: (result: OptimizationResult) => void;

	constructor(onNewModelReceived: (result: OptimizationResult) => void) {
		this.onNewModelReceived = onNewModelReceived;
	}

	connect = () => {
		if (!browser || this.socket) return;

		this.socket_status = ConnectionStatus.Connecting;
		this.socket = new WebSocket(WEBSOCKET_URL);
		this.socket.onopen = () => {
			this.socket_status = ConnectionStatus.Connected;
		};

		this.socket.onmessage = (event) => {
			const message = JSON.parse(event.data) as ServerMessage;
			this.lastMessage = message;

			switch (message.type) {
				case ServerMessageType.Setup:
					this.optimizationStatus = OptimizationStatus.Setup;
					break;
				case ServerMessageType.Grounding:
					this.optimizationStatus = OptimizationStatus.Grounding;
					break;
				case ServerMessageType.Solving:
					this.optimizationStatus = OptimizationStatus.Solving;
					break;
				case ServerMessageType.Finished:
					this.optimizationStatus = OptimizationStatus.Finished;
					break;
				case ServerMessageType.Canceled:
					this.optimizationStatus = OptimizationStatus.Ready;
					break;
				case ServerMessageType.Error:
					this.optimizationStatus = OptimizationStatus.Ready;
					break;
				case ServerMessageType.NewModel:
					this.onNewModelReceived(message.data);
					break;
			}

			console.log('Received message:', message);
		};

		this.socket.onclose = () => {
			this.socket_status = ConnectionStatus.Disconnected;
			this.optimizationStatus = OptimizationStatus.Ready;
		};
	};

	disconnect = () => {
		if (this.socket) {
			this.socket.close();
			this.socket = null;
		}
	};

	send = (message: ClientMessage) => {
		if (this.socket && this.socket_status === ConnectionStatus.Connected) {
			this.socket?.send(JSON.stringify(message));
		}
	};
}
