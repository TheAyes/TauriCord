// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
	compatibilityDate: "2025-01-13",
	devtools: { enabled: true },
	vite: {
		css: {
			preprocessorOptions: {
				scss: {
					compiler: "modern"
				}
			}
		}
	}
});
