declare module 'piston-client' {
	interface Api {
		runtimes(): Promise<Record<string, any>>;
		execute(lang: string, code: string): Promise<Record<string, any>>;
	}

	const piston: ({ server }: { server: string }) => Api;

	export = piston;
}
