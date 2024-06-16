import { Emulator } from "./nes";

(async () => {
	const emulator = await Emulator.createEmulator(Buffer.alloc(64 * 1024));
	emulator.step();
})()
	.catch(e => {
		console.error("fatal", e);
		process.exit(1);
	});
