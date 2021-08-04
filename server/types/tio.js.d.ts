declare module 'tio.js' {
	function tio(code: string, language?: string, timeout?: number): Promise<Record<string, any>>;
	namespace tio {
		export function setDefaultTimeout(timeout?: number): void;
	}

	export default tio;
}
