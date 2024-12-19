import { Logger } from "./index";

const logger = new Logger("new", {
	level: "debug",
});

for (let i = 0; i < 10; i++) {
	logger.info("info");
	logger.warn("warn");
	logger.error(new Error("error").toString());
	logger.debug("debug");
}
