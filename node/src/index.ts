import { Logger, consoleLogger } from "./logger";
import { Emulator, SingleBankAllRAMMemoryController } from "./nes/node";

const logger = new Logger([consoleLogger]);
(async () => {
	const emulator = await Emulator.createEmulator({
		initialMemory: Buffer.alloc(64 * 1024),
		memoryController: new SingleBankAllRAMMemoryController(),
		logger,
	});
	emulator.step();
})()
	.catch(e => {
		logger.fatal("fatal", e);
		process.exit(1);
	});
