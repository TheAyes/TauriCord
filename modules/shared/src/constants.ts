export const PLATFORM_MAP = {
	linux: "linux",
	macos: "mac",
	windows: "windows"
} as const;

const getPlatform = (platformKey: keyof typeof PLATFORM_MAP) =>
	navigator.userAgent.toLowerCase().includes(PLATFORM_MAP[platformKey]);

export const IS_PLATFORM = {
	LINUX: getPlatform("linux"),
	MACOS: getPlatform("macos"),
	WINDOWS: getPlatform("windows")
} as const;

export const HOSTNAME_MAP = {
	stable: "discord.com",
	canary: "canary.discord.com",
	ptb: "ptb.discord.com"
} as const;
