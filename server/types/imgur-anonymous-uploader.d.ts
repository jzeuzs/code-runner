declare module 'imgur-anonymous-uploader' {
	class ImgurAnonymousUploader {
		public clientId: string;

		public constructor(clientId: string);

		public uploadBuffer(buffer: Buffer): Promise<{ url: string }>;
	}

	export = ImgurAnonymousUploader;
}
