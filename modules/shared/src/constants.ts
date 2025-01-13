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

const VersionString = `AppleWebKit/537.36 (KHTML, like Gecko) Chrome/${process.versions.chrome?.split(".")[0]}.0.0.0 Safari/537.36`;
export const BrowserUserAgents = {
	darwin: `Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) ${VersionString}`,
	linux: `Mozilla/5.0 (X11; Linux x86_64) ${VersionString}`,
	windows: `Mozilla/5.0 (Windows NT 10.0; Win64; x64) ${VersionString}`
} as const;
