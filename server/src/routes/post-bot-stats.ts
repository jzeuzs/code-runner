import type { FastifyInstance } from 'fastify';
import { fetch, FetchResultTypes } from '@sapphire/fetch';

export default (app: FastifyInstance, _: any, done: () => void) => {
	app.post('/', async (req, reply) => {
		const body = req.body as Record<string, number>;
		const count = body.server_count;

		await fetch(
			'https://api.infinitybotlist.com/bot/871593892280160276',
			{
				headers: {
					authorization: process.env.INFINITY_KEY!,
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					servers: count
				}),
				method: 'POST'
			},
			FetchResultTypes.JSON
		);

		await fetch(
			'https://top.gg/api/bots/871593892280160276/stats',
			{
				headers: {
					Authorization: process.env.TOP_GG!,
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({
					server_count: count
				}),
				method: 'POST'
			},
			FetchResultTypes.JSON
		);

		reply.status(204);
	});

	done();
};
