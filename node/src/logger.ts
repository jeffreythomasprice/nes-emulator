export enum LogLevel {
	TRACE,
	DEBUG,
	INFO,
	WARN,
	ERROR,
	FATAL,
}

const LOG_LEVEL_STRINGS = {
	[LogLevel.TRACE]: "TRACE",
	[LogLevel.DEBUG]: "DEBUG",
	[LogLevel.INFO]: "INFO ",
	[LogLevel.WARN]: "WARN ",
	[LogLevel.ERROR]: "ERROR",
	[LogLevel.FATAL]: "FATAL",
} as const;

export type LogFunc = (level: LogLevel, message: string) => void;

export class Logger {
	private readonly destinations: ReadonlyArray<LogFunc>;

	constructor(destinations: LogFunc[]) {
		this.destinations = [...destinations];
	}

	log(level: LogLevel, message: string, ...args: unknown[]) {
		message = [message]
			.concat(...args.map(arg => {
				if (arg instanceof Error) {
					return arg.stack ?? arg.message;
				}
				return `${arg}`;
			}))
			.join(" ");
		this.destinations.forEach(f => f(level, message));
	}

	trace(message: string, ...args: unknown[]) {
		this.log(LogLevel.TRACE, message, ...args);
	}

	debug(message: string, ...args: unknown[]) {
		this.log(LogLevel.DEBUG, message, ...args);
	}

	info(message: string, ...args: unknown[]) {
		this.log(LogLevel.INFO, message, ...args);
	}

	warn(message: string, ...args: unknown[]) {
		this.log(LogLevel.WARN, message, ...args);
	}

	error(message: string, ...args: unknown[]) {
		this.log(LogLevel.ERROR, message, ...args);
	}

	fatal(message: string, ...args: unknown[]) {
		this.log(LogLevel.FATAL, message, ...args);
	}
}

export const consoleLogger: LogFunc = (level, message) => {
	message = `${new Date().toISOString()} [${LOG_LEVEL_STRINGS[level]}]: ${message}`;
	if (level <= LogLevel.INFO) {
		console.log(message);
	} else {
		console.error(message);
	}
};
