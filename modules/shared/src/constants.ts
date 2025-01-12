const getPlatform = (platform: string) => navigator.userAgent.includes(platform.toLowerCase());

export const IS_NIXOS = getPlatform("Nixos");

export const DISCORD_HOSTNAMES = ["discord.com", "canary.discord.com", "ptb.discord.com"];

const VersionString = `AppleWebKit/537.36 (KHTML, like Gecko) Chrome/${process.versions.chrome?.split(".")[0]}.0.0.0 Safari/537.36`;
const BrowserUserAgents = {
	darwin: `Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) ${VersionString}`,
	linux: `Mozilla/5.0 (X11; Linux x86_64) ${VersionString}`,
	windows: `Mozilla/5.0 (Windows NT 10.0; Win64; x64) ${VersionString}`
};
