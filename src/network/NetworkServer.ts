/**
 *  _   _      _
 * | \ | |    | |
 * |  \| | ___| |_ _ __ _____  __
 * | . ` |/ _ \ __| '__/ _ \ \/ /
 * | |\  |  __/ |_| | |  __/>  <
 * |_| \_|\___|\__|_|  \___/_/\_\
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * @author Netrex Team
 * @link https://github.com/NetrexMC
 *
 * © Netrex 2020 - 2021
 */
import Connection from "./common/Connection.ts";
import { EventEmitter } from "https://deno.land/std@0.97.0/node/events.ts";

export enum NetworkType {
	RakNet,
	DLTS,
	WS,
	UNKNOWN
}

export enum NetworkEventType {
	GamePacket = "packet_game",
	Disconnect = "disconnect"
}

export default abstract class NetworkServer {
	/**
	 * The Network type.
	 */
	public abstract serverType: NetworkType;

	/**
	 * The events channel.
	 */
	public abstract channel: EventEmitter;

	/**
	 * Starts the Network Server
	 */
	public abstract start(...args: any[]): any;

	/**
	 * Stops the Network Server
	 */
	public abstract stop(): any;

	/**
	 * Send a buffer to a specific client.
	 */
	public abstract send(...args: any[]): any;

	/**
	 * Gets the current connections
	 */
	public abstract get connections(): any;//Connection[];
}