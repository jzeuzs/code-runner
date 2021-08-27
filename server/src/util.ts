import type Redis from 'ioredis';
import type { FastifyInstance } from 'fastify';
import logger from 'consola';
import { readdir } from 'fs/promises';

export const getCache = async (redis: Redis.Redis, lang: string, code: string) => {
	const cached = await redis.get(`${lang}-${code}`).catch(() => null);

	if (!cached) return null;

	return JSON.parse(cached);
};

export const setCache = (redis: Redis.Redis, lang: string, code: string, data: Record<string, string>) =>
	redis.setex(`${lang}-${code}`, 604800, JSON.stringify(data));

export const loadRoutes = async (app: FastifyInstance) => {
	const files = await readdir('./dist/routes');

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
