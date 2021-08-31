import type Redis from 'ioredis';
import type { FastifyInstance } from 'fastify';
import logger from 'consola';
import { readdir } from 'fs/promises';
import { Readable } from 'stream';
import { fetch, FetchResultTypes } from '@sapphire/fetch';
import FormData from 'form-data';

export const enum Seconds {
	WEEK = 604_800,
	MONTH = 2_629_800
}

export const getCache = async (redis: Redis.Redis, lang: string, code: string) => {
	const cached = await redis.get(`${lang}-${code}`).catch(() => null);

	if (!cached) return null;

	return JSON.parse(cached);
};

export const setCache = (redis: Redis.Redis, lang: string, code: string, data: Record<string, string>) =>
	redis.setex(`${lang}-${code}`, Seconds.WEEK, JSON.stringify(data));

export const loadRoutes = async (app: FastifyInstance) => {
	const files = await readdir('./server/dist/routes');

	for (const file of files.filter((f) => f.endsWith('.js'))) {
		const name = file.split('.')[0];
		const route = await import(`#routes/${name}`);

		logger.info(`[${name}] - Route Loaded`);

		if (name === 'run') app.register(route, { prefix: '/' });
		else app.register(route, { prefix: name });
	}
};

export const trimArray = (arr: string[]) => {
	if (arr.length > 10) {
		const len = arr.length - 10;

		arr = arr.slice(0, 10);
		arr.push(`and ${len} more...`);
	}

	return arr;
};

export const bufferToStream = (buffer: Buffer) => {
	const stream = new Readable({
		read() {
			this.push(buffer);
			this.push(null);
		}
	});

	return stream;
};

export const uploadImage = async (stream: Readable, redis: Redis.Redis, code: string) => {
	const cached = await redis.get(`format-${code}`);

	if (cached) return cached;

	const form = new FormData();

	form.append('file', stream);

	const {
		data: { direct_url }
	} = await fetch<Record<string, Record<string, string>>>(
		'https://api.tixte.com/v1/upload',
		{
			headers: {
				Authorization: process.env.UPLOAD_AUTH!,
				domain: 'i.tomio.codes',
				'Content-Type': 'application/x-www-form-urlencoded'
			},
			body: form,
			method: 'POST'
		},
		FetchResultTypes.JSON
	);

	await redis.set(`format-${code}`, direct_url);

	return direct_url;
};
