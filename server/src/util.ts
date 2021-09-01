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

export const bufferToStream = (buffer: Buffer) => Readable.from(buffer.toString());

export const uploadImage = async (file: Buffer, redis: Redis.Redis, code: string) => {
	const cached = await redis.get(`format-${code}`);

	if (cached) return cached;

	const form = new FormData();

	form.append('image', file.toString('base64'));

	const {
		data: { link }
	} = await fetch<Record<string, Record<string, string>>>(
		'https://api.imgur.com/3/image',
		{
			headers: {
				Authorization: `Client-ID ${process.env.IMGUR_CLIENT_ID}`,
				'Content-Type': 'multipart/form-data',
				...form.getHeaders()
			},
			body: form,
			method: 'POST'
		},
		FetchResultTypes.JSON
	);

	await redis.set(`format-${code}`, link);

	return link;
};
